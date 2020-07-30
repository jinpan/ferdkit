#include "GraphMol/SmilesParse/SmilesWrite.h"

extern "C"

bool in_orgo_subset(int atomic_num) {
  return RDKit::SmilesWrite::inOrganicSubset(atomic_num);
}