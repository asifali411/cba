use std::fs;
use std::path::Path;

pub fn find_files(root: impl AsRef<Path>, pattern: &str) -> Vec<String> {
  let root = root.as_ref();

  let pattern = pattern.strip_prefix('/').unwrap_or(pattern);

  let recursive = pattern.contains("**");

  let mut result = Vec::new();

  if recursive {
    collect_recursive(root, root, pattern, &mut result);
  } else {
    collect_direct(root, root, pattern, &mut result);
  }

  result
}

fn collect_recursive(root: &Path, current: &Path, pattern: &str, result: &mut Vec<String>) {
  let Ok(entries) = fs::read_dir(current) else {
    return;
  };

  for entry in entries.flatten() {
    let path = entry.path();

    if path.is_dir() {
      collect_recursive(root, &path, pattern, result);
    } else if path.is_file() {
      if let Ok(relative) = path.strip_prefix(root) {
        let relative = relative.to_string_lossy().replace('\\', "/");

        if glob_match(pattern, &relative) {
          result.push(path.to_string_lossy().replace('\\', "/"));
        }
      }
    }
  }
}

fn collect_direct(root: &Path, base: &Path, pattern: &str, result: &mut Vec<String>) {
  let Ok(entries) = fs::read_dir(base) else {
    return;
  };

  for entry in entries.flatten() {
    let path = entry.path();

    if !path.is_file() {
      continue;
    }

    if let Ok(relative) = path.strip_prefix(root) {
      let relative = relative.to_string_lossy().replace('\\', "/");

      if glob_match(pattern, &relative) {
        result.push(path.to_string_lossy().replace('\\', "/"));
      }
    }
  }
}
fn glob_match(pattern: &str, path: &str) -> bool {
  let pattern = pattern.as_bytes();
  let path = path.as_bytes();

  fn matches(pattern: &[u8], path: &[u8], pi: usize, si: usize) -> bool {
    if pi == pattern.len() {
      return si == path.len();
    }

    if pattern[pi] == b'*' && pi + 1 < pattern.len() && pattern[pi + 1] == b'*' {
      let mut p = pi + 2;
      while p + 1 < pattern.len() && pattern[p] == b'*' && pattern[p + 1] == b'*' {
        p += 2;
      }

      if p < pattern.len() && pattern[p] == b'/' {
        if matches(pattern, path, p + 1, si) {
          return true;
        }
      } else if matches(pattern, path, p, si) {
        return true;
      }

      if si < path.len() && matches(pattern, path, pi, si + 1) {
        return true;
      }

      return false;
    }

    if pattern[pi] == b'*' {
      if matches(pattern, path, pi + 1, si) {
        return true;
      }

      if si < path.len() && path[si] != b'/' && matches(pattern, path, pi, si + 1) {
        return true;
      }

      return false;
    }

    if pattern[pi] == b'?' {
      return si < path.len() && path[si] != b'/' && matches(pattern, path, pi + 1, si + 1);
    }

    if pattern[pi] == b'[' {
      if let Some((matched, next)) = match_class(pattern, path, pi, si) {
        if matched {
          return matches(pattern, path, next, si + 1);
        }

        return false;
      }
    }

    if si < path.len() && pattern[pi] == path[si] {
      return matches(pattern, path, pi + 1, si + 1);
    }

    false
  }

  fn match_class(pattern: &[u8], path: &[u8], pi: usize, si: usize) -> Option<(bool, usize)> {
    if si >= path.len() || path[si] == b'/' {
      return Some((false, pi));
    }

    let mut p = pi + 1;

    if p >= pattern.len() {
      return None;
    }

    let negated = match pattern[p] {
      b'!' | b'^' => {
        p += 1;
        true
      }
      _ => false,
    };

    let mut matched = false;
    let mut has_content = false;

    while p < pattern.len() && pattern[p] != b']' {
      has_content = true;

      if p + 2 < pattern.len() && pattern[p + 1] == b'-' && pattern[p + 2] != b']' {
        let start = pattern[p];
        let end = pattern[p + 2];

        if start <= path[si] && path[si] <= end {
          matched = true;
        }

        p += 3;
      } else {
        if pattern[p] == path[si] {
          matched = true;
        }

        p += 1;
      }
    }

    if p >= pattern.len() || !has_content {
      return None;
    }

    Some(((if negated { !matched } else { matched }), p + 1))
  }

  matches(pattern, path, 0, 0)
}
