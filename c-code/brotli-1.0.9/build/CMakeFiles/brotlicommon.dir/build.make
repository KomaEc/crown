# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.16

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:


#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:


# Remove some rules from gmake that .SUFFIXES does not remove.
SUFFIXES =

.SUFFIXES: .hpux_make_needs_suffix_list


# Suppress display of executed commands.
$(VERBOSE).SILENT:


# A target that is always out of date.
cmake_force:

.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/bin/cmake

# The command to remove a file.
RM = /usr/bin/cmake -E remove -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /home/vagrant/orc-benchmark/brotli-1.0.9

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /home/vagrant/orc-benchmark/brotli-1.0.9/build

# Include any dependencies generated for this target.
include CMakeFiles/brotlicommon.dir/depend.make

# Include the progress variables for this target.
include CMakeFiles/brotlicommon.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/brotlicommon.dir/flags.make

CMakeFiles/brotlicommon.dir/c/common/constants.c.o: CMakeFiles/brotlicommon.dir/flags.make
CMakeFiles/brotlicommon.dir/c/common/constants.c.o: ../c/common/constants.c
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/vagrant/orc-benchmark/brotli-1.0.9/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building C object CMakeFiles/brotlicommon.dir/c/common/constants.c.o"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -o CMakeFiles/brotlicommon.dir/c/common/constants.c.o   -c /home/vagrant/orc-benchmark/brotli-1.0.9/c/common/constants.c

CMakeFiles/brotlicommon.dir/c/common/constants.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/brotlicommon.dir/c/common/constants.c.i"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/vagrant/orc-benchmark/brotli-1.0.9/c/common/constants.c > CMakeFiles/brotlicommon.dir/c/common/constants.c.i

CMakeFiles/brotlicommon.dir/c/common/constants.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/brotlicommon.dir/c/common/constants.c.s"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/vagrant/orc-benchmark/brotli-1.0.9/c/common/constants.c -o CMakeFiles/brotlicommon.dir/c/common/constants.c.s

CMakeFiles/brotlicommon.dir/c/common/context.c.o: CMakeFiles/brotlicommon.dir/flags.make
CMakeFiles/brotlicommon.dir/c/common/context.c.o: ../c/common/context.c
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/vagrant/orc-benchmark/brotli-1.0.9/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Building C object CMakeFiles/brotlicommon.dir/c/common/context.c.o"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -o CMakeFiles/brotlicommon.dir/c/common/context.c.o   -c /home/vagrant/orc-benchmark/brotli-1.0.9/c/common/context.c

CMakeFiles/brotlicommon.dir/c/common/context.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/brotlicommon.dir/c/common/context.c.i"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/vagrant/orc-benchmark/brotli-1.0.9/c/common/context.c > CMakeFiles/brotlicommon.dir/c/common/context.c.i

CMakeFiles/brotlicommon.dir/c/common/context.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/brotlicommon.dir/c/common/context.c.s"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/vagrant/orc-benchmark/brotli-1.0.9/c/common/context.c -o CMakeFiles/brotlicommon.dir/c/common/context.c.s

CMakeFiles/brotlicommon.dir/c/common/dictionary.c.o: CMakeFiles/brotlicommon.dir/flags.make
CMakeFiles/brotlicommon.dir/c/common/dictionary.c.o: ../c/common/dictionary.c
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/vagrant/orc-benchmark/brotli-1.0.9/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_3) "Building C object CMakeFiles/brotlicommon.dir/c/common/dictionary.c.o"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -o CMakeFiles/brotlicommon.dir/c/common/dictionary.c.o   -c /home/vagrant/orc-benchmark/brotli-1.0.9/c/common/dictionary.c

CMakeFiles/brotlicommon.dir/c/common/dictionary.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/brotlicommon.dir/c/common/dictionary.c.i"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/vagrant/orc-benchmark/brotli-1.0.9/c/common/dictionary.c > CMakeFiles/brotlicommon.dir/c/common/dictionary.c.i

CMakeFiles/brotlicommon.dir/c/common/dictionary.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/brotlicommon.dir/c/common/dictionary.c.s"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/vagrant/orc-benchmark/brotli-1.0.9/c/common/dictionary.c -o CMakeFiles/brotlicommon.dir/c/common/dictionary.c.s

CMakeFiles/brotlicommon.dir/c/common/platform.c.o: CMakeFiles/brotlicommon.dir/flags.make
CMakeFiles/brotlicommon.dir/c/common/platform.c.o: ../c/common/platform.c
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/vagrant/orc-benchmark/brotli-1.0.9/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_4) "Building C object CMakeFiles/brotlicommon.dir/c/common/platform.c.o"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -o CMakeFiles/brotlicommon.dir/c/common/platform.c.o   -c /home/vagrant/orc-benchmark/brotli-1.0.9/c/common/platform.c

CMakeFiles/brotlicommon.dir/c/common/platform.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/brotlicommon.dir/c/common/platform.c.i"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/vagrant/orc-benchmark/brotli-1.0.9/c/common/platform.c > CMakeFiles/brotlicommon.dir/c/common/platform.c.i

CMakeFiles/brotlicommon.dir/c/common/platform.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/brotlicommon.dir/c/common/platform.c.s"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/vagrant/orc-benchmark/brotli-1.0.9/c/common/platform.c -o CMakeFiles/brotlicommon.dir/c/common/platform.c.s

CMakeFiles/brotlicommon.dir/c/common/transform.c.o: CMakeFiles/brotlicommon.dir/flags.make
CMakeFiles/brotlicommon.dir/c/common/transform.c.o: ../c/common/transform.c
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/vagrant/orc-benchmark/brotli-1.0.9/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_5) "Building C object CMakeFiles/brotlicommon.dir/c/common/transform.c.o"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -o CMakeFiles/brotlicommon.dir/c/common/transform.c.o   -c /home/vagrant/orc-benchmark/brotli-1.0.9/c/common/transform.c

CMakeFiles/brotlicommon.dir/c/common/transform.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/brotlicommon.dir/c/common/transform.c.i"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/vagrant/orc-benchmark/brotli-1.0.9/c/common/transform.c > CMakeFiles/brotlicommon.dir/c/common/transform.c.i

CMakeFiles/brotlicommon.dir/c/common/transform.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/brotlicommon.dir/c/common/transform.c.s"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/vagrant/orc-benchmark/brotli-1.0.9/c/common/transform.c -o CMakeFiles/brotlicommon.dir/c/common/transform.c.s

# Object files for target brotlicommon
brotlicommon_OBJECTS = \
"CMakeFiles/brotlicommon.dir/c/common/constants.c.o" \
"CMakeFiles/brotlicommon.dir/c/common/context.c.o" \
"CMakeFiles/brotlicommon.dir/c/common/dictionary.c.o" \
"CMakeFiles/brotlicommon.dir/c/common/platform.c.o" \
"CMakeFiles/brotlicommon.dir/c/common/transform.c.o"

# External object files for target brotlicommon
brotlicommon_EXTERNAL_OBJECTS =

libbrotlicommon.so.1.0.9: CMakeFiles/brotlicommon.dir/c/common/constants.c.o
libbrotlicommon.so.1.0.9: CMakeFiles/brotlicommon.dir/c/common/context.c.o
libbrotlicommon.so.1.0.9: CMakeFiles/brotlicommon.dir/c/common/dictionary.c.o
libbrotlicommon.so.1.0.9: CMakeFiles/brotlicommon.dir/c/common/platform.c.o
libbrotlicommon.so.1.0.9: CMakeFiles/brotlicommon.dir/c/common/transform.c.o
libbrotlicommon.so.1.0.9: CMakeFiles/brotlicommon.dir/build.make
libbrotlicommon.so.1.0.9: CMakeFiles/brotlicommon.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir=/home/vagrant/orc-benchmark/brotli-1.0.9/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_6) "Linking C shared library libbrotlicommon.so"
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/brotlicommon.dir/link.txt --verbose=$(VERBOSE)
	$(CMAKE_COMMAND) -E cmake_symlink_library libbrotlicommon.so.1.0.9 libbrotlicommon.so.1 libbrotlicommon.so

libbrotlicommon.so.1: libbrotlicommon.so.1.0.9
	@$(CMAKE_COMMAND) -E touch_nocreate libbrotlicommon.so.1

libbrotlicommon.so: libbrotlicommon.so.1.0.9
	@$(CMAKE_COMMAND) -E touch_nocreate libbrotlicommon.so

# Rule to build all files generated by this target.
CMakeFiles/brotlicommon.dir/build: libbrotlicommon.so

.PHONY : CMakeFiles/brotlicommon.dir/build

CMakeFiles/brotlicommon.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/brotlicommon.dir/cmake_clean.cmake
.PHONY : CMakeFiles/brotlicommon.dir/clean

CMakeFiles/brotlicommon.dir/depend:
	cd /home/vagrant/orc-benchmark/brotli-1.0.9/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/vagrant/orc-benchmark/brotli-1.0.9 /home/vagrant/orc-benchmark/brotli-1.0.9 /home/vagrant/orc-benchmark/brotli-1.0.9/build /home/vagrant/orc-benchmark/brotli-1.0.9/build /home/vagrant/orc-benchmark/brotli-1.0.9/build/CMakeFiles/brotlicommon.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : CMakeFiles/brotlicommon.dir/depend

