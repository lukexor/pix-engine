<!-- markdownlint-disable-file no-duplicate-heading -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

### 🐛 Bug Fixes


- Problem around chrono, time, and an CVE - ([fb4a22f](https://github.com/lukexor/pix-engine/commit/fb4a22fab9a2c8b4d6d9b3707fce9edddbaf9386))
- Fixed build badge - ([0a01708](https://github.com/lukexor/pix-engine/commit/0a0170866ba4fe16b1d01a88a46ce0970a2a53b2))
- Changes for PR after code review - ([2896d25](https://github.com/lukexor/pix-engine/commit/2896d256967bdc4302b0f9495e97cc72fb6026f4))
- Changes for PR after code review - ([bc42596](https://github.com/lukexor/pix-engine/commit/bc4259681630bc36cb5a50f481f285919f35c084))

### 📚 Documentation


- Fixed MSRV in readme - ([032f7b1](https://github.com/lukexor/pix-engine/commit/032f7b10d4356859b456544d3ba88d85c2356ad1))

### 🎨 Styling


- Fix format - ([3695546](https://github.com/lukexor/pix-engine/commit/36955467e136f11283922393372df60850e31771))

### ⚙️ Miscellaneous Tasks


- Fix ci - ([9fb7c74](https://github.com/lukexor/pix-engine/commit/9fb7c74604b6787fdbbf8f33243f17a555b1e706))
- Updated dependencies - ([4dd3c51](https://github.com/lukexor/pix-engine/commit/4dd3c5120d98e8d2dc09c74ee310281b30d35f4b))
- Updated Cargo.lock - ([cf68729](https://github.com/lukexor/pix-engine/commit/cf68729ad8f8a9bef9c93fc344ca2957046df97f))
- Remove nightly and beta from CI - ([5d60e1f](https://github.com/lukexor/pix-engine/commit/5d60e1fa150670f8c22fb7e03952955ded8c2986))

## [0.7.0](https://github.com/lukexor/pix-engine/compare/v0.6.0..v0.7.0) - 2023-01-20

### ⛰️  Features


- Add getters for window position - ([d488091](https://github.com/lukexor/pix-engine/commit/d48809116b1d20690cf6ae7e2d2599949a92af09))
- Add getters for window position - ([bc29778](https://github.com/lukexor/pix-engine/commit/bc29778528ad26532c2168b7686182cc8a66d010))

### 🐛 Bug Fixes


- Proceed without cursor support - ([c4baf4d](https://github.com/lukexor/pix-engine/commit/c4baf4dd2974732fd8e704e33691a49b7b476312))
- Proceed without cursor support - ([fc3b038](https://github.com/lukexor/pix-engine/commit/fc3b0380f56a5e93d18375e7e218164a4b03a545))
- Fixed PixResult/PixError in examples - ([21b76e2](https://github.com/lukexor/pix-engine/commit/21b76e204976a51e80a3e71be84efae6bfd73019))
- Improved rendering to window and texture targets - ([1ecdd3a](https://github.com/lukexor/pix-engine/commit/1ecdd3af55a352f2407a106c086d80356cac5c09))
- Removed unnecessary clone - ([a3aafe1](https://github.com/lukexor/pix-engine/commit/a3aafe1f23f0f1cc24830ea4ba8a16b866a8c14a))

### 🚜 Refactor


- Revert PixError/PixResult change - ([2fe7f73](https://github.com/lukexor/pix-engine/commit/2fe7f73e115c23995bc6b45f4dcef64db38debb7))
- Removed with_ prefix from builder methods - ([92e6899](https://github.com/lukexor/pix-engine/commit/92e689996dd31f7459d2ad8166b87bedee713dbf))
- Renamed primary trait - ([fd12ea5](https://github.com/lukexor/pix-engine/commit/fd12ea56a3c5471417d4161e8b16cd6fa0460394))
- Renamed types - ([e8b7a43](https://github.com/lukexor/pix-engine/commit/e8b7a43339ee52d56fe37242ebfc9c0a09390948))
- Change example loop traversal - ([907a94d](https://github.com/lukexor/pix-engine/commit/907a94d894a8d1a5e7713b3cb66e90701c1aa234))
- Avoid calcualting target_delta_time each frame - ([df57bda](https://github.com/lukexor/pix-engine/commit/df57bda2a9a5579fb1f2e1c312709f67dd70c9e7))
- Change Unsupported to Unhandled - ([8529133](https://github.com/lukexor/pix-engine/commit/8529133948fe97bf6780d5c9e20c5c0974e0e872))

### 📚 Documentation


- Updated readme - ([32b08d9](https://github.com/lukexor/pix-engine/commit/32b08d9d88c75bbc05f0569a92775eb83fffa786))
- Updated keywords - ([43abd77](https://github.com/lukexor/pix-engine/commit/43abd77decc93da857d44c6d6d836a7b8133f28b))
- Updated changelog - ([8483b59](https://github.com/lukexor/pix-engine/commit/8483b596df8499cfc6b19c50731c57b5c0b9cb69))
- Updated README - ([7c6cee2](https://github.com/lukexor/pix-engine/commit/7c6cee220ee4b357ed923a029086f9a95bcbd60d))

### 🎨 Styling


- Cleaned up some lints - ([387b68f](https://github.com/lukexor/pix-engine/commit/387b68f10c5093d7734757bea6c7e801b65e1ece))
- Fixed nightly lints - ([56482b2](https://github.com/lukexor/pix-engine/commit/56482b26884d233f0c676cb8ea1f5dcc0669ecf1))
- Fixed nightly lints - ([5e8aea4](https://github.com/lukexor/pix-engine/commit/5e8aea4330daa9303433be9b94d029c61ec2dc54))

### 🧪 Testing


- Remove vimspeector for global config - ([b4f7e1a](https://github.com/lukexor/pix-engine/commit/b4f7e1a3297bbec9ff99b63dd16f9ad7f90469b9))
- Ignore README examples - ([662b2d9](https://github.com/lukexor/pix-engine/commit/662b2d9eebac71ee5c42d5e22243045f4c417df0))

### ⚙️ Miscellaneous Tasks


- Fix keyword length - ([c25f49f](https://github.com/lukexor/pix-engine/commit/c25f49f446ac0f52f9a94387f83a741d187b5309))
- Tag 0.7.0 again - ([487e225](https://github.com/lukexor/pix-engine/commit/487e225128d87446f7d0d5a5c7c6e45624fd660e))
- Increased msrv to 1.62.0 - ([b66c38c](https://github.com/lukexor/pix-engine/commit/b66c38cd7f1541c4ce46622fdefaa499166e4c1a))
- Updated dependencies - ([72ccf96](https://github.com/lukexor/pix-engine/commit/72ccf968be1cab62847449e0a7a736a97ba2680a))
- Updated Cargo.lock - ([420251e](https://github.com/lukexor/pix-engine/commit/420251ea7c1a4029448509f49d50b8507da6a12d))
- Publish 0.7.0 - ([e1983a2](https://github.com/lukexor/pix-engine/commit/e1983a2f7987571d506e1c12fd443b02c954e4f7))
- Update dependencies - ([0fd9f00](https://github.com/lukexor/pix-engine/commit/0fd9f00cf60aeaddb1c7c1a644565e5c6859d54f))
- Update lru - ([961b689](https://github.com/lukexor/pix-engine/commit/961b6894e6cc953d68cec6088f5aa65fbfdd7449))
- Updated packages - ([3a49859](https://github.com/lukexor/pix-engine/commit/3a498591085d40f965e9c0be75356438735dda66))
- Fix line continuation - ([b4d6cae](https://github.com/lukexor/pix-engine/commit/b4d6cae2e3dfe2db0de10f57ebb2446746e8dca9))
- Change coverage format - ([cdb5597](https://github.com/lukexor/pix-engine/commit/cdb5597aaae68cf664ec511b09e9a45060c68a2b))
- Fix serde feature - ([99c17c3](https://github.com/lukexor/pix-engine/commit/99c17c3cc5cdbf7971fead0ee2bb3f866e842d60))
- Provide coverage file - ([8f3e7d6](https://github.com/lukexor/pix-engine/commit/8f3e7d6ba4784e3fe5382029d313c029b0200b9e))
- Fix demangler - ([98d4dc5](https://github.com/lukexor/pix-engine/commit/98d4dc568be0a9d9ef21964b1e2f83b92f4b7e93))
- Try removing read - ([e6fe1a9](https://github.com/lukexor/pix-engine/commit/e6fe1a95c928d0eded0dbd05be3573f9a2f002fe))
- Fix coverage - ([e47c2ad](https://github.com/lukexor/pix-engine/commit/e47c2ad45f7ba4b6368eac712a4dcb707497613c))
- Add debugging - ([46b4a4b](https://github.com/lukexor/pix-engine/commit/46b4a4b83f935665a41a9da969267bb3c84dcce1))
- Test rust-cov - ([bca8028](https://github.com/lukexor/pix-engine/commit/bca80287e87eb514e9d6bb2d26cf777f2d56f71b))
- Another attempt to fix coverage - ([a272015](https://github.com/lukexor/pix-engine/commit/a27201504c3cae0f8d70c817414f48d87c6e71e5))
- Fix env variable - ([161b05d](https://github.com/lukexor/pix-engine/commit/161b05dbdbd4ed61227c4bea2e832d9d5fe6138e))
- Fix sudo condition - ([053147c](https://github.com/lukexor/pix-engine/commit/053147c1438500ec0d2696d9e08ba484b894a220))
- Fix condition - ([a39c4cc](https://github.com/lukexor/pix-engine/commit/a39c4cc37eacd8af3479817697ef7ba0562c4d47))
- Add conditional sudo - ([d620727](https://github.com/lukexor/pix-engine/commit/d6207270ac133afc4754f0884bfb2307aeb0a0d2))
- Fix brew path - ([df8a898](https://github.com/lukexor/pix-engine/commit/df8a898a80a0b0ec018a3fe273c0cd3b6086e341))
- Removed sudo - ([5517a3f](https://github.com/lukexor/pix-engine/commit/5517a3fc7e3983ba1610a8d3133e07ee8943c7d1))
- Fix actions - ([ab062b6](https://github.com/lukexor/pix-engine/commit/ab062b6f6be5ce957ad356d4b9021bde16daf4a6))
- Changed to composite actions - ([d55be29](https://github.com/lukexor/pix-engine/commit/d55be29baf2657b875dc712fa93e4bca2ea8ea5a))
- Fix workflows - ([9983f55](https://github.com/lukexor/pix-engine/commit/9983f5589883ee54613f751134d9da78ed00bbf2))
- Refactor workflows - ([1c64ac7](https://github.com/lukexor/pix-engine/commit/1c64ac7b221d22441034fe08adf94d5778b4a7c5))
- Try and fix doctest coverage - ([ada1b07](https://github.com/lukexor/pix-engine/commit/ada1b073a0af7c1c5ff9765f051412a00dbf5692))
- Added triage workflow - ([4a999cd](https://github.com/lukexor/pix-engine/commit/4a999cd3799bc6039550866c597fe66229caaae4))
- Update dependencies - ([31cad3b](https://github.com/lukexor/pix-engine/commit/31cad3b06186c318a506f74b5642670e12a89ded))
- Cleanup yaml - ([436003a](https://github.com/lukexor/pix-engine/commit/436003af0bc155069fe3af4874670652bff10c80))
- Fixed coverege workflow - ([be4aeec](https://github.com/lukexor/pix-engine/commit/be4aeecba1a83a18d0ae47a1180dca23f6b2f6eb))
- Disable coverage for now - ([f150b25](https://github.com/lukexor/pix-engine/commit/f150b25837ec1c5c0fa54cfd472a21b47ce4bfd7))
- Updated Cargo.lock - ([478f0f8](https://github.com/lukexor/pix-engine/commit/478f0f8a9278e82f3162ee3cdecf98b9954035d4))
- Use container for coverage - ([21001a1](https://github.com/lukexor/pix-engine/commit/21001a113f398293427708b3e35cf5983a96e79a))
- Moved to coverage workflow - ([8539ffc](https://github.com/lukexor/pix-engine/commit/8539ffcab93a559913bac2008902b8aa16fd15c2))
- Fix run types - ([9a518f9](https://github.com/lukexor/pix-engine/commit/9a518f98f611d8613fcb2cc09086775a7474a673))
- Add all targets to coverage - ([bd75c18](https://github.com/lukexor/pix-engine/commit/bd75c18632c8dab706403b329af5e9b2172ac123))
- Updated Cargo.lock - ([7fc8cc3](https://github.com/lukexor/pix-engine/commit/7fc8cc3d8746617fe757906b1136d46cd99893ab))
- Combined code coverage - ([90b1d2a](https://github.com/lukexor/pix-engine/commit/90b1d2aea6eceaa7e3ebff6cd9f4bf6272343ddc))
- Added coverage workflow - ([bc58a26](https://github.com/lukexor/pix-engine/commit/bc58a2689bca98dd2ccd843807be447a1b25a9f6))
- Fix linuxbrew cache - ([64af38a](https://github.com/lukexor/pix-engine/commit/64af38ab7b2ca0fbdd2d011b36679a0d9a82d67a))
- Clean up ci - ([31960e3](https://github.com/lukexor/pix-engine/commit/31960e321c5d63dbbc9001d1faa3b0e0a5f142de))
- Update package workflow - ([1bdd93b](https://github.com/lukexor/pix-engine/commit/1bdd93b1beca5c8fe91a61817bf07efc4d09e626))

### ◀️ Revert


- Reverted consuming builder - ([f76f972](https://github.com/lukexor/pix-engine/commit/f76f9727b996e50813e2ea5d19c28721de7d0529))

## [0.6.0](https://github.com/lukexor/pix-engine/compare/v0.5.4..v0.6.0) - 2022-06-20

### ⛰️  Features

- *(audio)* [**breaking**] Support multiple channel types for audio and added wasm checks back in - ([f431f8f](https://github.com/lukexor/pix-engine/commit/f431f8f703666167b5c3183fae49adc09da69f6d))
- *(audio)* [**breaking**] Added PixState::audio_queued_size and PixState::audio_size - ([3b364bc](https://github.com/lukexor/pix-engine/commit/3b364bcfbcccb06016eab28223e5962bd289100a))

- Added arrow navigation to select_box - ([a80b665](https://github.com/lukexor/pix-engine/commit/a80b665140a93518d073c8100bdd97b70a5402cc))
- Added contains for Point to triangle - ([f5f4154](https://github.com/lukexor/pix-engine/commit/f5f41540eb51f7c2e731f6171657990d167d7e92))

### 🐛 Bug Fixes

- *(audio)* Allow device defaults for audio spec and fix audio pause - ([c3e40d9](https://github.com/lukexor/pix-engine/commit/c3e40d9b68bb0b7afdba7f9beb72a75332919c5f))
- *(audio)* Set default audio buffer size to 4096 - ([5ff3509](https://github.com/lukexor/pix-engine/commit/5ff3509021460bbd1c622c928af14653c5beba83))
- *(engine)* Fixed frame rate epsilon - ([e74334f](https://github.com/lukexor/pix-engine/commit/e74334f6cf8770fd3bbd5f6d1ed8ab586ea7f58f))

- Fixed fluid_simulation clearing on every frame - ([93605c3](https://github.com/lukexor/pix-engine/commit/93605c3220e6d71e4466754b87826c36e801b85f))
- Fluid_simulation window size - ([4d429b6](https://github.com/lukexor/pix-engine/commit/4d429b6fb8f02099c608f39058c70e3484b11da6))
- Fixed setting font affecting theme - ([e995fe4](https://github.com/lukexor/pix-engine/commit/e995fe4ef3f559d17d034eef79c630b9fb635376))
- Fixed select_box focusing - ([7123dc6](https://github.com/lukexor/pix-engine/commit/7123dc64bbce202226b42b86cba106b6fd52c03e))
- Fixed select_box expansion when focused - ([6262cba](https://github.com/lukexor/pix-engine/commit/6262cba47e3f961be51fcaa224278e5ccf0ad30b))
- [**breaking**] Remove vcpkg - ([f824388](https://github.com/lukexor/pix-engine/commit/f8243884d53f412a7f405df19f41155a551b8656))
- Fixed forcing updates to sleep if no target frame rate is defined - ([192c50b](https://github.com/lukexor/pix-engine/commit/192c50bc9b20b3a21bbccfbe921b6561c7899e07))
- [**breaking**] Fixed nightly lints by removing serialization from Font - ([60681a6](https://github.com/lukexor/pix-engine/commit/60681a68a5702091a0a1dbb2ad8a6f706487b9a7))
- Don't debug print font bytes - ([b0f67d1](https://github.com/lukexor/pix-engine/commit/b0f67d13ccac62c628e7680a7e16fe9dd5ef8675))

### 🚜 Refactor


- Swapped lazy_static for once_cell - ([05a4100](https://github.com/lukexor/pix-engine/commit/05a4100e622a4e3a13832e3c53c47f6d6783b2cf))
- [**breaking**] Renamed as_array and as_byte methods to coords and points - ([b839788](https://github.com/lukexor/pix-engine/commit/b8397889cdf370f0633b5b36fe8763759615c7ad))
- Reduce ui state property nesting - ([7fbe33d](https://github.com/lukexor/pix-engine/commit/7fbe33daf9cc3f8b6f4746805c16689ee5750caf))
- [**breaking**] Unified scale functionality to be limited only to rendering and not window size - ([0a763b5](https://github.com/lukexor/pix-engine/commit/0a763b5a80b47aebcb2e699a69aae42a6599f185))
- Move vsync condition up - ([35223f4](https://github.com/lukexor/pix-engine/commit/35223f4c3f3968863bd352d93632faff0f63eb20))
- Lint pass - ([0e754db](https://github.com/lukexor/pix-engine/commit/0e754db1b760e49ce686120d6c870500ec4b8d12))

### 📚 Documentation


- Updated README - ([39d43c5](https://github.com/lukexor/pix-engine/commit/39d43c5f5b4a3e1937117a6fd8b346ba650aba8a))
- Update README - ([69d5951](https://github.com/lukexor/pix-engine/commit/69d5951f24ef0e8506198de848ae101d3066c56f))
- Fixed lints - ([ebcd983](https://github.com/lukexor/pix-engine/commit/ebcd983da6f1306e03e96de110ad7895f3cf689c))
- Clarified audio buffer size documentation - ([c4b88ca](https://github.com/lukexor/pix-engine/commit/c4b88ca361ac7dde3ccfe05486f7c663d67f507c))

### 🎨 Styling


- Fix lints - ([ae27f9d](https://github.com/lukexor/pix-engine/commit/ae27f9d8eab8e7bcc0f8d39d7f447e9c884b1d2e))
- Fixed nightly lint - ([7dfbd9a](https://github.com/lukexor/pix-engine/commit/7dfbd9a77617adbaf09792822fcb558596c2fee7))

### ⚙️ Miscellaneous Tasks


- Fix package.yml - ([2d641f7](https://github.com/lukexor/pix-engine/commit/2d641f7141d975479571a05335ffafd3cb539c0a))
- Release 0.6.0 - ([b41cbd6](https://github.com/lukexor/pix-engine/commit/b41cbd6bb7c630fb898e097be1d2c856a7007973))
- Update Cargo.lock - ([de8dbed](https://github.com/lukexor/pix-engine/commit/de8dbed978742112531a5abdb1a2c0b1a255fede))
- Update CHANGELOG - ([5a89b1e](https://github.com/lukexor/pix-engine/commit/5a89b1e8e2a0ebc0a16fcb57890c5a8658857512))
- Update license badge - ([6fd70b0](https://github.com/lukexor/pix-engine/commit/6fd70b063829d5fa2382b9dcc373b14ade629cfd))
- Fix ci shell - ([4969a95](https://github.com/lukexor/pix-engine/commit/4969a95cc3680ae037d06e9975cec6fb67b10449))
- Fail early in scripts - ([63c2ca3](https://github.com/lukexor/pix-engine/commit/63c2ca37e84d90b50ad5e966e53d731494a78c7a))
- Fixed commit-msg - ([8de117d](https://github.com/lukexor/pix-engine/commit/8de117d445d532984b8045506a8d2794cf5e204a))
- Add DLLs - ([8d1811e](https://github.com/lukexor/pix-engine/commit/8d1811e4645bb1c4e9dcc5ffe26eb12f2256ab31))
- Fix windows-latest - ([2858e7a](https://github.com/lukexor/pix-engine/commit/2858e7a0c0a5b4f54c024210bb9134f21326d89c))
- Fix windows-latest - ([1a3c8d4](https://github.com/lukexor/pix-engine/commit/1a3c8d40bb98517e6ab38800e0438f3cc18da4c1))
- Fixed commit-msg - ([7229d52](https://github.com/lukexor/pix-engine/commit/7229d5203d1d4bfacc28e650969aa7d5f92f80c7))
- Changed to custom hooks - ([e66fe66](https://github.com/lukexor/pix-engine/commit/e66fe6615b3f59627d3e6597e34b79a80316dba3))
- Added SDL libs for windows - ([fd7beeb](https://github.com/lukexor/pix-engine/commit/fd7beebcf96af1cc59c3d363f4b16b76df8709af))
- Revert quote change - ([ffeefb3](https://github.com/lukexor/pix-engine/commit/ffeefb39bac1a5c5dc5d036d9bcd59986345c7d5))
- WIP on providing SDL dependencies for windows build - ([e06c239](https://github.com/lukexor/pix-engine/commit/e06c239853db744f03db2324a28f25a2b8c44529))
- Set LD_LIBRARY_PATH - ([5a82753](https://github.com/lukexor/pix-engine/commit/5a82753945711dbc29a3dc95b66a1c662024ba21))
- Revert feature change - ([1b71017](https://github.com/lukexor/pix-engine/commit/1b71017e6463374d5090da040e8c55cabf71f38b))
- Remove empty feature run - ([729740f](https://github.com/lukexor/pix-engine/commit/729740f64f0133307b05dc0f45089da77790826b))
- Another attempt - ([f9843df](https://github.com/lukexor/pix-engine/commit/f9843df552f87a578f2eed3112ca480fc7444eeb))
- More debug - ([ec23086](https://github.com/lukexor/pix-engine/commit/ec2308691ef5358cb89a18a453a44c588ec8c28b))
- Test ci debug - ([3ee6a54](https://github.com/lukexor/pix-engine/commit/3ee6a548c057437a348e93dadf2216bc37ea0a40))
- Use LD_LIBRARY_PATH - ([15f3fc5](https://github.com/lukexor/pix-engine/commit/15f3fc5d434cbd8e95dbe845511a5d26e0ae90b9))
- Update changelog and ci - ([929ec6c](https://github.com/lukexor/pix-engine/commit/929ec6c6e0072e6227c4135f52d2a6c948347e34))
- Updated Cargo.lock - ([7939ddd](https://github.com/lukexor/pix-engine/commit/7939ddde0b49b52517e76aed79870f5c5063c94d))
- Try bash - ([5603b65](https://github.com/lukexor/pix-engine/commit/5603b65d4faa255bf742579e168b081c14992572))
- Attempt using homebrew for linux and windows - ([8b40187](https://github.com/lukexor/pix-engine/commit/8b40187cdbcc9cabd1046de4e0930f909c820b73))
- Updated Cargo.lock - ([39dcde3](https://github.com/lukexor/pix-engine/commit/39dcde32a74718a163c24af4cd95131d2d5d4505))
- Updated Cargo.lock - ([cf79418](https://github.com/lukexor/pix-engine/commit/cf79418d3edced6c64832ffe77e43f43e38d1540))
- Revert back to vcpkg - ([80b014a](https://github.com/lukexor/pix-engine/commit/80b014a7eb0a28e044905d123c032940d92d0e69))
- Try adding universe - ([7cf6a80](https://github.com/lukexor/pix-engine/commit/7cf6a8087b4426cd1801af0c856d3725cc337261))
- Fix features - ([0878620](https://github.com/lukexor/pix-engine/commit/0878620398c9280233e69ef9f0d3904ec3f28807))
- Fix Cargo.toml - ([137d4ff](https://github.com/lukexor/pix-engine/commit/137d4ff9c8b0e47913c73bfb2ce32b632ff84279))
- Fix ci - ([d081940](https://github.com/lukexor/pix-engine/commit/d0819407a79c28e181850bfbeb9e1acf8b676c7c))
- Stop iusing vcpkg for linux and macos - ([2207faf](https://github.com/lukexor/pix-engine/commit/2207faf3a1c4d9d09043716d969bd26367c2a41d))
- Try another version - ([f924ee2](https://github.com/lukexor/pix-engine/commit/f924ee2081e768c5562d0fca911eca7d1c9e10ec))
- Try an earlier version - ([15250dd](https://github.com/lukexor/pix-engine/commit/15250dd3fcbd05e8da7469aa9b37298f37dd538c))
- Try updating vcpkg - ([b11f207](https://github.com/lukexor/pix-engine/commit/b11f2075579e8d4675e0088460c3c25666129ac7))
- Update Cargo.lock - ([32e5994](https://github.com/lukexor/pix-engine/commit/32e5994daf19f5d359188ff61d1b9876251add29))

<!-- generated by git-cliff -->
