pub mod rbxcdn;
pub mod bindings;
pub mod error;

static PLAYER_IGNORE_MANIFEST_PACKAGES: [&str; 1] = [
	"RobloxPlayerLauncher.exe",
];
static STUDIO_IGNORE_MANIFEST_PACKAGES: [&str; 1] = [
	"RobloxStudioLauncherBeta.exe",
];

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn get_latest_version_hash() {
		let version = rbxcdn::get_latest_version(rbxcdn::Binary::Player, None).unwrap();
		println!("Got latest version: {}", version);
		assert_eq!(version.len(), 24);
	}
	#[test]
	fn get_latest_version_hash_from_channel() {
		let version_player = rbxcdn::get_latest_version(rbxcdn::Binary::Player, Some("LIVE")).unwrap();
		let version_studio = rbxcdn::get_latest_version(rbxcdn::Binary::Player, Some("LIVE")).unwrap();
		assert_eq!(version_player.len(), 24);
		assert_eq!(version_studio.len(), 24);
	}
	#[test]
	#[should_panic]
	fn get_latest_version_hash_from_restricted_channel() {
		rbxcdn::get_latest_version(rbxcdn::Binary::Player, Some("zbeta")).unwrap();
	}
	#[test]
	fn get_manifest() {
		let version = rbxcdn::get_latest_version(rbxcdn::Binary::Player, None).unwrap();
		let manifest = rbxcdn::get_manifest(version).unwrap();

		for package in &manifest.contents {
			let package_name = &package.package_name;
			if PLAYER_IGNORE_MANIFEST_PACKAGES.contains(&package_name.as_str()) {
				continue;
			}

			if bindings::PLAYER_EXTRACT_BINDINGS.iter().any(|(name, _)| name == package_name) { // blame ethan
				continue;
			} else {
				panic!("Unknown package: {} on Player bindings", package_name);
			}
		}

		let version = rbxcdn::get_latest_version(rbxcdn::Binary::Studio, None).unwrap();
		let manifest = rbxcdn::get_manifest(version).unwrap();

		for package in &manifest.contents {
			let package_name = &package.package_name;
			if STUDIO_IGNORE_MANIFEST_PACKAGES.contains(&package_name.as_str()) {
				continue;
			}

			if bindings::STUDIO_EXTRACT_BINDINGS.iter().any(|(name, _)| name == package_name) {
				continue;
			} else {
				panic!("Unknown package: {} on Studio bindings", package_name);
			}
		}
	}
}