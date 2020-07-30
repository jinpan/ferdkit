libwrapper: export RDBASE=../rdkit
libwrapper: export RDLIB=../rdkit/lib/
libwrapper: export LD_LIBRARY_PATH=../rdkit/lib/

main: export RDBASE=../rdkit
main: export RDLIB=../rdkit/lib/
main: export LD_LIBRARY_PATH=../rdkit/lib/

libwrapper:
	clang++ -O3 -std=c++11 -shared -fPIC -I$$RDBASE/Code wrapper.cc -o libwrapper.so -L$$RDLIB -lRDKitSmilesParse

main: libwrapper
	clang -O3 main.c -o main -L. -lwrapper
	# run main as LD_LIBRARY_PATH=.:../rdkit/lib ./main
	# note: need to have previously built rdkit from source via their cmake.
