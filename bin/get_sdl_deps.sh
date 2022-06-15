#/bin/bash

set -xeuo pipefail

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

# sdlMinGWArchive="SDL2-devel-$sdlVersion-mingw.tar.gz"
# sdlMinGWImageArchive="SDL2_image-devel-$sdlImageVersion-mingw.tar.gz"
# sdlMinGWMixerArchive="SDL2_mixer-devel-$sdlMixerVersion-mingw.tar.gz";
# sdlMinGWTtfArchive="SDL2_ttf-devel-$sdlTtfVersion-mingw.tar.gz";


# URLs

sdlVCUrl="https://www.libsdl.org/release/$sdlVCArchive"
sdlVCImageUrl="https://www.libsdl.org/projects/SDL_image/release/$sdlVCImageArchive"
sdlVCMixerUrl="https://www.libsdl.org/projects/SDL_mixer/release/$sdlVCMixerArchive"
sdlVCTtfUrl="https://www.libsdl.org/projects/SDL_ttf/release/$sdlVCTtfArchive"

# sdlMinGWUrl="https://www.libsdl.org/release/$sdlMinGWArchive"
# sdlMinGWImageUrl="https://www.libsdl.org/projects/SDL_image/release/$sdlMinGWImageArchive"
# sdlMinGWMixerUrl="https://www.libsdl.org/projects/SDL_mixer/release/$sdlMinGWMixerArchive"
# sdlMinGWTtfUrl="https://www.libsdl.org/projects/SDL_ttf/release/$sdlMinGWTtfArchive"

# Download

curl -sSLfO $sdlVCUrl
curl -sSLfO $sdlVCImageUrl
curl -sSLfO $sdlVCMixerUrl
curl -sSLfO $sdlVCTtfUrl

# curl -sSLfO $sdlMinGWUrl
# curl -sSLfO $sdlMinGWImageUrl
# curl -sSLfO $sdlMinGWMixerUrl
# curl -sSLfO $sdlMinGWTtfUrl

# Build SDL2_gfx

# if [ ! -d vcpkg ]; then
#   git clone https://github.com/microsoft/vcpkg
#   ./vcpkg/bootstrap-vcpkg.sh
# fi
# pushd vcpkg;
# git pull
# ./vcpkg install sdl2-gfx sdl2-gfx:x64-windows

# Extract

sdlFolder="SDL2-$sdlVersion"
sdlGfxFolder="vcpkg/packages/"
sdlImageFolder="SDL2_image-$sdlImageVersion"
sdlMixerFolder="SDL2_mixer-$sdlMixerVersion"
sdlTtfFolder="SDL2_ttf-$sdlTtfVersion"

unzip -n $sdlVCArchive
unzip -n $sdlVCImageArchive
unzip -n $sdlVCMixerArchive
unzip -n $sdlVCTtfArchive

# tar kxf $sdlMinGWArchive
# tar kxf $sdlMinGWImageArchive
# tar kxf $sdlMinGWMixerArchive
# tar kxf $sdlMinGWTtfArchive

# Make destination folders

MSVCDLL32=msvc/dll/32
MSVCDLL64=msvc/dll/64
MSVCLib32=msvc/lib/32
MSVCLib64=msvc/lib/64
# MinGWDLL32=gnu-mingw/dll/32
# MinGWDLL64=gnu-mingw/dll/64
# MinGWLib32=gnu-mingw/lib/32
# MinGWLib64=gnu-mingw/lib/64

mkdir -p $MSVCDLL32
mkdir -p $MSVCDLL64
mkdir -p $MSVCLib32
mkdir -p $MSVCLib64
# mkdir -p $MinGWDLL32
# mkdir -p $MinGWDLL64
# mkdir -p $MinGWLib32
# mkdir -p $MinGWLib64


# Copy

MSVCDLLFiles32=lib/x86/*.dll
MSVCDLLFiles64=lib/x64/*.dll
MSVCLibFiles32=lib/x86/SDL2_*.lib
MSVCLibFiles64=lib/x64/SDL2_*.lib
# MinGWDLLFiles32=i686-w64-mingw32/bin/*
# MinGWDLLFiles64=x86_64-w64-mingw32/bin/*
# MinGWLibFiles32=i686-w64-mingw32/lib/SDL2_*
# MinGWLibFiles64=x86_64-w64-mingw32/lib/SDL2_*

MSVCGfxDLLFiles32=sdl2-gfx_x86-windows/bin/*.dll
MSVCGfxDLLFiles64=sdl2-gfx_x64-windows/bin/*.dll
MSVCGfxLibFiles32=sdl2-gfx_x86-windows/lib/SDL2_*.lib
MSVCGfxLibFiles64=sdl2-gfx_x64-windows/lib/SDL2_*.lib
# MinGWGfxDLLFiles32=sdl2-gfx_x86-mingw-dynamic/bin/*
# MinGWGfxDLLFiles64=sdl2-gfx_x64-mingw-dynamic/bin/*
# MinGWGfxLibFiles32=sdl2-gfx_x86-mingw-dynamic/lib/*
# MinGWGfxLibFiles64=sdl2-gfx_x64-mingw-dynamic/lib/*

# cp -v $sdlGfxFolder/$MSVCDLLFiles32 $MSVCDLL32
# cp -v $sdlGfxFolder/$MSVCDLLFiles64 $MSVCDLL64
# cp -v $sdlGfxFolder/$MSVCLibFiles32 $MSVCLib32
# cp -v $sdlGfxFolder/$MSVCLibFiles64 $MSVCLib64
# cp -v $sdlGfxFolder/$MinGWDLLFiles32 $MinGWDLL32
# cp -v $sdlGfxFolder/$MinGWDLLFiles64 $MinGWDLL64
# cp -v $sdlGfxFolder/$MinGWLibFiles32 $MinGWLib32
# cp -v $sdlGfxFolder/$MinGWLibFiles64 $MinGWLib64

cp -v $sdlFolder/$MSVCDLLFiles32 $MSVCDLL32
cp -v $sdlFolder/$MSVCDLLFiles64 $MSVCDLL64
cp -v $sdlFolder/$MSVCLibFiles32 $MSVCLib32
cp -v $sdlFolder/$MSVCLibFiles64 $MSVCLib64
# cp -v $sdlFolder/$MinGWDLLFiles32 $MinGWDLL32
# cp -v $sdlFolder/$MinGWDLLFiles64 $MinGWDLL64
# cp -v $sdlFolder/$MinGWLibFiles32 $MinGWLib32
# cp -v $sdlFolder/$MinGWLibFiles64 $MinGWLib64

cp -v $sdlImageFolder/$MSVCDLLFiles32 $MSVCDLL32
cp -v $sdlImageFolder/$MSVCDLLFiles64 $MSVCDLL64
cp -v $sdlImageFolder/$MSVCLibFiles32 $MSVCLib32
cp -v $sdlImageFolder/$MSVCLibFiles64 $MSVCLib64
# cp -v $sdlImageFolder/$MinGWDLLFiles32 $MinGWDLL32
# cp -v $sdlImageFolder/$MinGWDLLFiles64 $MinGWDLL64
# cp -v $sdlImageFolder/$MinGWLibFiles32 $MinGWLib32
# cp -v $sdlImageFolder/$MinGWLibFiles64 $MinGWLib64

cp -v $sdlMixerFolder/$MSVCDLLFiles32 $MSVCDLL32
cp -v $sdlMixerFolder/$MSVCDLLFiles64 $MSVCDLL64
cp -v $sdlMixerFolder/$MSVCLibFiles32 $MSVCLib32
cp -v $sdlMixerFolder/$MSVCLibFiles64 $MSVCLib64
# cp -v $sdlMixerFolder/$MinGWDLLFiles32 $MinGWDLL32
# cp -v $sdlMixerFolder/$MinGWDLLFiles64 $MinGWDLL64
# cp -v $sdlMixerFolder/$MinGWLibFiles32 $MinGWLib32
# cp -v $sdlMixerFolder/$MinGWLibFiles64 $MinGWLib64

cp -v $sdlTtfFolder/$MSVCDLLFiles32 $MSVCDLL32
cp -v $sdlTtfFolder/$MSVCDLLFiles64 $MSVCDLL64
cp -v $sdlTtfFolder/$MSVCLibFiles32 $MSVCLib32
cp -v $sdlTtfFolder/$MSVCLibFiles64 $MSVCLib64
# cp -v $sdlTtfFolder/$MinGWDLLFiles32 $MinGWDLL32
# cp -v $sdlTtfFolder/$MinGWDLLFiles64 $MinGWDLL64
# cp -v $sdlTtfFolder/$MinGWLibFiles32 $MinGWLib32
# cp -v $sdlTtfFolder/$MinGWLibFiles64 $MinGWLib64
