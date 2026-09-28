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
  let pattern: Vec<char> = pattern.chars().collect();
  let path: Vec<char> = path.chars().collect();

  fn match_recursive(pattern: &[char], path: &[char], pi: usize, si: usize) -> bool {
    if pi == pattern.len() {
      return si == path.len();
    }

    if pattern[pi] == '*' && pi + 1 < pattern.len() && pattern[pi + 1] == '*' {
      let mut next = pi + 2;
      while next + 1 < pattern.len() && pattern[next] == '*' && pattern[next + 1] == '*' {
        next += 2;
      }

      if match_recursive(pattern, path, next, si) {
        return true;
      }

      if si < path.len() && match_recursive(pattern, path, pi, si + 1) {
        return true;
      }

      return false;
    }

    if pattern[pi] == '*' {
      if match_recursive(pattern, path, pi + 1, si) {
        return true;
      }

      if si < path.len() && path[si] != '/' && match_recursive(pattern, path, pi, si + 1) {
        return true;
      }

      return false;
    }

    if pattern[pi] == '?' {
      return si < path.len() && path[si] != '/' && match_recursive(pattern, path, pi + 1, si + 1);
    }

    if si < path.len() && pattern[pi] == path[si] {
      return match_recursive(pattern, path, pi + 1, si + 1);
    }

    false
  }

  match_recursive(&pattern, &path, 0, 0)
}
