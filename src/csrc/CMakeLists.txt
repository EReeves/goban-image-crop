cmake_minimum_required(VERSION 3.0.0)
project(testlib VERSION 0.1.0)

include(CTest)
enable_testing()

file(GLOB SOURCES
    ./*.h
    ./*.cpp
)

find_package( OpenCV REQUIRED )
include_directories( ${OpenCV_INCLUDE_DIRS} )
add_executable(testlib test.c ${SOURCES})
target_link_libraries( testlib ${OpenCV_LIBS} )



set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)
