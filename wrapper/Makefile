clean:
	rm -f libwrapper.so libwrapper.a main

libwrapper:
	clang++ -c -I../rdkit/Code wrapper.cc -o libwrapper.so
	llvm-ar -rc libwrapper.a libwrapper.so
	rm libwrapper.so

main: libwrapper
	clang main.c -o main \
		-L. -L../rdkit/lib \
		-lstdc++ -lm \
		-lwrapper \
		-lRDKitSmilesParse_static \
		-lRDKitGraphMol_static \
		-lRDKitRDGeneral_static \
		-lRDKitRDGeometryLib_static \
		-lRDKitRingDecomposerLib_static \
		-lRDKitDataStructs_static
