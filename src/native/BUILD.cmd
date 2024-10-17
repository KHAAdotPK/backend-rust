@rem https://stackoverflow.com/questions/3155492/how-do-i-specify-the-platform-for-msbuild
@rem /p is short for /property
@rem msbuild project.xml /p:Configuration=Debug /p:Platform=x64
@rem msbuild project.xml
@rem msbuild start.xml /p:Configuration=Debug /p:Platform=x64

msbuild project.xml /p:Configuration=Debug /p:Platform=x64
