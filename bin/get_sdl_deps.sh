#/bin/bash

set -euo pipefail

MINGW=false

# Versions

sdlVersion="2.0.22"
sdlGfxVersion="1.0.4"
sdlImageVersion="2.0.5"
sdlMixerVersion="2.0.4"
sdlTtfVersion="2.0.18"

# Archive Files

sdlVCArchive="SDL2-devel-$sdlVersion-VC.zip"
sdlVCImageArchive="SDL2_image-devel-$sdlImageVersion-VC.zip"
sdlVCMixerArchive="SDL2_mixer-devel-$sdlMixerVersion-VC.zip";
sdlVCTtfArchive="SDL2_ttf-devel-$sdlTtfVersion-VC.zip";

$MINGW && sdlMinGWArchive="SDL2-devel-$sdlVersion-mingw.tar.gz"
$MINGW && sdlMinGWImageArchive="SDL2_image-devel-$sdlImageVersion-mingw.tar.gz"
$MINGW && sdlMinGWMixerArchive="SDL2_mixer-devel-$sdlMixerVersion-mingw.tar.gz";
$MINGW && sdlMinGWTtfArchive="SDL2_ttf-devel-$sdlTtfVersion-mingw.tar.gz";


# URLs

sdlVCUrl="https://www.libsdl.org/release/$sdlVCArchive"
sdlVCImageUrl="https://www.libsdl.org/projects/SDL_image/release/$sdlVCImageArchive"
sdlVCMixerUrl="https://www.libsdl.org/projects/SDL_mixer/release/$sdlVCMixerArchive"
sdlVCTtfUrl="https://www.libsdl.org/projects/SDL_ttf/release/$sdlVCTtfArchive"

$MINGW && sdlMinGWUrl="https://www.libsdl.org/release/$sdlMinGWArchive"
$MINGW && sdlMinGWImageUrl="https://www.libsdl.org/projects/SDL_image/release/$sdlMinGWImageArchive"
$MINGW && sdlMinGWMixerUrl="https://www.libsdl.org/projects/SDL_mixer/release/$sdlMinGWMixerArchive"
$MINGW && sdlMinGWTtfUrl="https://www.libsdl.org/projects/SDL_ttf/release/$sdlMinGWTtfArchive"

# Download

curl -sSLfO $sdlVCUrl
curl -sSLfO $sdlVCImageUrl
curl -sSLfO $sdlVCMixerUrl
curl -sSLfO $sdlVCTtfUrl

$MINGW && curl -sSLfO $sdlMinGWUrl
$MINGW && curl -sSLfO $sdlMinGWImageUrl
$MINGW && curl -sSLfO $sdlMinGWMixerUrl
$MINGW && curl -sSLfO $sdlMinGWTtfUrl

# Build SDL2_gfx

if [ ! -d vcpkg ]; then
  git clone https://github.com/microsoft/vcpkg
  ./vcpkg/bootstrap-vcpkg.sh
fi
pushd vcpkg
git pull
./vcpkg install sdl2-gfx sdl2-gfx:x64-windows
$MINGW && ./vcpkg install --triplet=x64-mingw-dynamic --host-triplet=x64-mingw-dynamic sdl2-gfx
$MINGW && ./vcpkg install --triplet=x86-mingw-dynamic --host-triplet=x86-mingw-dynamic sdl2-gfx
popd

# Extract

sdlFolder="SDL2-$sdlVersion"
sdlGfxFolder="vcpkg/packages"
sdlImageFolder="SDL2_image-$sdlImageVersion"
sdlMixerFolder="SDL2_mixer-$sdlMixerVersion"
sdlTtfFolder="SDL2_ttf-$sdlTtfVersion"

unzip -n $sdlVCArchive
unzip -n $sdlVCImageArchive
unzip -n $sdlVCMixerArchive
unzip -n $sdlVCTtfArchive

$MINGW && tar --keep-newer-files -xf $sdlMinGWArchive
$MINGW && tar --keep-newer-files -xf $sdlMinGWImageArchive
$MINGW && tar --keep-newer-files -xf $sdlMinGWMixerArchive
$MINGW && tar --keep-newer-files -xf $sdlMinGWTtfArchive

# Make destination folders

MSVCDLL32=lib/msvc/dll/32
MSVCDLL64=lib/msvc/dll/64
MSVCLib32=lib/msvc/lib/32
MSVCLib64=lib/msvc/lib/64
$MINGW && MinGWDLL32=lib/gnu-mingw/dll/32
$MINGW && MinGWDLL64=lib/gnu-mingw/dll/64
$MINGW && MinGWLib32=lib/gnu-mingw/lib/32
$MINGW && MinGWLib64=lib/gnu-mingw/lib/64

mkdir -p $MSVCDLL32
mkdir -p $MSVCDLL64
mkdir -p $MSVCLib32
mkdir -p $MSVCLib64
$MINGW && mkdir -p $MinGWDLL32
$MINGW && mkdir -p $MinGWDLL64
$MINGW && mkdir -p $MinGWLib32
$MINGW && mkdir -p $MinGWLib64


# Copy

MSVCDLLFiles32=lib/x86/*.dll
MSVCDLLFiles64=lib/x64/*.dll
MSVCLibFiles32=lib/x86/SDL2*.lib
MSVCLibFiles64=lib/x64/SDL2*.lib
$MINGW && MinGWDLLFiles32=i686-w64-mingw32/bin/*
$MINGW && MinGWDLLFiles64=x86_64-w64-mingw32/bin/*
$MINGW && MinGWLibFiles32=i686-w64-mingw32/lib/SDL2*
$MINGW && MinGWLibFiles64=x86_64-w64-mingw32/lib/SDL2*

MSVCGfxDLLFiles32=sdl2-gfx_x86-windows/bin/*.dll
MSVCGfxDLLFiles64=sdl2-gfx_x64-windows/bin/*.dll
MSVCGfxLibFiles32=sdl2-gfx_x86-windows/lib/SDL2*.lib
MSVCGfxLibFiles64=sdl2-gfx_x64-windows/lib/SDL2*.lib
$MINGW && MinGWGfxDLLFiles32=sdl2-gfx_x86-mingw-dynamic/bin/*
$MINGW && MinGWGfxDLLFiles64=sdl2-gfx_x64-mingw-dynamic/bin/*
$MINGW && MinGWGfxLibFiles32=sdl2-gfx_x86-mingw-dynamic/lib/*
$MINGW && MinGWGfxLibFiles64=sdl2-gfx_x64-mingw-dynamic/lib/*

cp -v $sdlGfxFolder/$MSVCGfxDLLFiles32 $MSVCDLL32
cp -v $sdlGfxFolder/$MSVCGfxDLLFiles64 $MSVCDLL64
cp -v $sdlGfxFolder/$MSVCGfxLibFiles32 $MSVCLib32
cp -v $sdlGfxFolder/$MSVCGfxLibFiles64 $MSVCLib64
$MINGW && cp -v $sdlGfxFolder/$MinGWGfxDLLFiles32 $MinGWDLL32
$MINGW && cp -v $sdlGfxFolder/$MinGWGfxLibFiles64 $MinGWDLL64
$MINGW && cp -v $sdlGfxFolder/$MinGWGfxDLLFiles32 $MinGWLib32
$MINGW && cp -v $sdlGfxFolder/$MinGWGfxLibFiles64 $MinGWLib64

cp -v $sdlFolder/$MSVCDLLFiles32 $MSVCDLL32
cp -v $sdlFolder/$MSVCDLLFiles64 $MSVCDLL64
cp -v $sdlFolder/$MSVCLibFiles32 $MSVCLib32
cp -v $sdlFolder/$MSVCLibFiles64 $MSVCLib64
$MINGW && cp -v $sdlFolder/$MinGWDLLFiles32 $MinGWDLL32
$MINGW && cp -v $sdlFolder/$MinGWDLLFiles64 $MinGWDLL64
$MINGW && cp -v $sdlFolder/$MinGWLibFiles32 $MinGWLib32
$MINGW && cp -v $sdlFolder/$MinGWLibFiles64 $MinGWLib64

cp -v $sdlImageFolder/$MSVCDLLFiles32 $MSVCDLL32
cp -v $sdlImageFolder/$MSVCDLLFiles64 $MSVCDLL64
cp -v $sdlImageFolder/$MSVCLibFiles32 $MSVCLib32
cp -v $sdlImageFolder/$MSVCLibFiles64 $MSVCLib64
$MINGW && cp -v $sdlImageFolder/$MinGWDLLFiles32 $MinGWDLL32
$MINGW && cp -v $sdlImageFolder/$MinGWDLLFiles64 $MinGWDLL64
$MINGW && cp -v $sdlImageFolder/$MinGWLibFiles32 $MinGWLib32
$MINGW && cp -v $sdlImageFolder/$MinGWLibFiles64 $MinGWLib64

cp -v $sdlMixerFolder/$MSVCDLLFiles32 $MSVCDLL32
cp -v $sdlMixerFolder/$MSVCDLLFiles64 $MSVCDLL64
cp -v $sdlMixerFolder/$MSVCLibFiles32 $MSVCLib32
cp -v $sdlMixerFolder/$MSVCLibFiles64 $MSVCLib64
$MINGW && cp -v $sdlMixerFolder/$MinGWDLLFiles32 $MinGWDLL32
$MINGW && cp -v $sdlMixerFolder/$MinGWDLLFiles64 $MinGWDLL64
$MINGW && cp -v $sdlMixerFolder/$MinGWLibFiles32 $MinGWLib32
$MINGW && cp -v $sdlMixerFolder/$MinGWLibFiles64 $MinGWLib64

cp -v $sdlTtfFolder/$MSVCDLLFiles32 $MSVCDLL32
cp -v $sdlTtfFolder/$MSVCDLLFiles64 $MSVCDLL64
cp -v $sdlTtfFolder/$MSVCLibFiles32 $MSVCLib32
cp -v $sdlTtfFolder/$MSVCLibFiles64 $MSVCLib64
$MINGW && cp -v $sdlTtfFolder/$MinGWDLLFiles32 $MinGWDLL32
$MINGW && cp -v $sdlTtfFolder/$MinGWDLLFiles64 $MinGWDLL64
$MINGW && cp -v $sdlTtfFolder/$MinGWLibFiles32 $MinGWLib32
$MINGW && cp -v $sdlTtfFolder/$MinGWLibFiles64 $MinGWLib64

# Cleanup

rm -rf SDL*