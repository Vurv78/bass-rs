
use cc::windows_registry;

use std::{
	env, ffi::OsStr,
	fs::File, io::Write,
	path::{Path, PathBuf},
	process::Command
};

fn main() {
	let out_dir = env::var("OUT_DIR").expect("Couldn't get OUT_DIR");
	let out_dir = Path::new( &out_dir );

	println!("cargo:rerun-if-changed=bass.h");

	let bindings = bindgen::Builder::default()
		.header("bass.h")
		.allowlist_function("BASS.*")
		.allowlist_type("BASS.*")
		.allowlist_var("BASS.*")
		.generate()
		.expect("Couldn't generate bindings!");

	bindings
		.write_to_file(out_dir.join("bindings.rs"))
		.expect("Couldn't write bindings!");

	link(&out_dir);
}

fn link(out_dir: &Path) {
	let target = env::var("TARGET").expect("Couldn't get `TARGET` env variable");
	let target_arch = env::var("CARGO_CFG_TARGET_ARCH").expect("Couldn't find target arch!");

	let dll_path = find_dll(&target_arch)
		.expect( "bass.dll couldn't be found! Try setting the environment variable BASS_DLL_PATH to where bass.dll is located" );

	assert_eq!(dll_path.extension().unwrap(), "dll");
	let stem = dll_path.file_stem().unwrap().to_str().unwrap();

	let lib_path = out_dir.join( format!("{}.lib", stem) );
	let def_path = out_dir.join( format!("{}.def", stem) );

	generate_def(windows_registry::find(&target, "dumpbin.exe").expect("Couldn't find dumpbin.exe"), &dll_path, &def_path)
		.expect("Failed to generate .def file from dll");

	let mut libtool = windows_registry::find(&target, "lib.exe").expect("Couldn't find lib.exe");

	assert!(
		libtool.arg("/NOLOGO")
		.arg(format!("/MACHINE:x{}", if target_arch == "x86_64" {"64"} else {"86"} ))
		.arg(format!("/DEF:{}", def_path.display()))
		.arg(format!("/OUT:{}", lib_path.display()))
		.status()
		.unwrap()
		.success()
	);

	println!("Linking to {}", dll_path.display()); // Will only show if you make cargo very verbose manually (Also it's bugged)

	println!( "cargo:rustc-link-search=native={}", out_dir.display() );
	println!( "cargo:rustc-link-lib=dylib={}", stem );
}

fn generate_def<P: AsRef<OsStr>, Q: AsRef<Path>> (
	mut dumpbin: Command,
	dll_path: P,
	def_path: Q
) -> Result<(), std::io::Error> {
	let dll_path = dll_path.as_ref();

	let output = dumpbin
		.arg("/EXPORTS")
		.arg(dll_path)
		.output()
		.unwrap();

	let stdout = String::from_utf8(output.stdout).unwrap();
	let mut handle = File::create(def_path)?;
	writeln!( handle, "EXPORTS" )?;

	stdout
		.split("\r\n")
		.map(|line| line.trim())
		.skip_while(|line| line != &"ordinal hint RVA      name")
		.skip(2)
		.take_while(|line| line != &"")
		.map(|line| line.split_whitespace().nth(3).unwrap())
		.for_each(|line| writeln!( &mut handle, "{}", line.to_string() ).unwrap() );
	handle.flush()?;

	Ok(())
}

#[cfg(not(feature = "garrysmod"))]
fn find_dll(_target_arch: &String) -> Option<PathBuf> {
	Some( PathBuf::from( env::var("BASS_DLL_PATH").ok()? ) )
}

#[cfg(feature = "garrysmod")]
fn find_dll(target_arch: &String) -> Option<PathBuf> {
	if let Ok(dll_path) = env::var("BASS_DLL_PATH") {
		return Some(PathBuf::from(dll_path));
	}

	let steam_paths: Vec<PathBuf> = [
		r#"C:\Program Files (x86)\Steam"#,
		r#"C:\SteamLibrary"#,
		r#"D:\SteamLibrary"#,
		r#"E:\SteamLibrary"#,
		r#"F:\SteamLibrary"#,
	]
	.iter()
	.map(|a| a.into())
	.collect();

	for steam_path in steam_paths {
		let mut dll_path = steam_path
			.join("steamapps")
			.join("common")
			.join("GarrysMod")
			.join("bin");

			if target_arch == "x86_64" { dll_path = dll_path.join("win64"); }

			dll_path = dll_path.join("bass.dll");

		if dll_path.is_file() {
			return Some(dll_path);
		}
	}

	None
}