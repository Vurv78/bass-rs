# ``bass-rs`` [![Release Shield](https://img.shields.io/github/v/release/Vurv78/bass-rs)](https://github.com/Vurv78/bass-rs/releases/latest) [![License](https://img.shields.io/github/license/Vurv78/bass-rs?color=red)](https://opensource.org/licenses/MIT) ![CI](https://github.com/Vurv78/bass-rs/actions/workflows/doc.yml/badge.svg)
Bindings to [BASS](https://www.un4seen.com).  

It will generate bindings using [bindgen](https://github.com/rust-lang/rust-bindgen) and then try to find a bass dll file at the ``BASS_DLL_PATH`` environment variable.  

You can set this variable with a batchfile or by using the experimental [\[env\]](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#configurable-env) tag on nightly cargo (Although I've never got it working).

It will statically link to this by generating a .lib file with msvc, so this will only work on Windows.  

## Notes
1. 32 bit is weird. Bindgen returns stdcalls instead of extern "C" which mangles functions incorrectly. Even after manually changing this I think it didn't work, but you are free to try

2. There is a ``garrysmod`` feature which will attempt to look in common steam install folders to find a bass dll from there. (This repo was originally made for this)

3. As said before this doesn't support anything but Windows to use MSVC.
