// This file defines the versioning logic, including functions to compare the current version with the latest available version.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Version { major, minor, patch }
    }

    pub fn from_str(version_str: &str) -> Option<Self> {
        let parts: Vec<&str> = version_str.split('.').collect();
        if parts.len() == 3 {
            if let (Ok(major), Ok(minor), Ok(patch)) = (
                parts[0].parse(),
                parts[1].parse(),
                parts[2].parse(),
            ) {
                return Some(Version::new(major, minor, patch));
            }
        }
        None
    }

    pub fn is_newer_than(&self, other: &Version) -> bool {
        if self.major > other.major {
            true
        } else if self.major == other.major {
            if self.minor > other.minor {
                true
            } else if self.minor == other.minor {
                self.patch > other.patch
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_comparison() {
        let v1 = Version::new(1, 0, 0);
        let v2 = Version::new(1, 1, 0);
        let v3 = Version::new(1, 1, 1);
        let v4 = Version::new(2, 0, 0);
        
        assert!(!v1.is_newer_than(&v1));
        assert!(v2.is_newer_than(&v1));
        assert!(v3.is_newer_than(&v2));
    }
}