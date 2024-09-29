https://mmcif.wwpdb.org/


There are two problems with the pdb/mmcif that could use immediate improvement: 
1. chemical completeness  (See https://github.com/openmm/pdbfixer)
2. and biological modularity (classification and access)


## Gameplan

TODO: read the mmcif format specification
TODO: read more of Matteo's answers on the matter on BSO?

Confine ourselves only to fields of the mmcif format that are relevant to specifying:

- atoms:
    - bonds
    - angles
    - dihedrals
    - impropers
    - nonbonded interactions
    - unit cell
    - symmetry operations
    - metadata

- residues:
    -   sequence
    -  secondary structure
    - metadata

- chains:
    - sequence
    - secondary structure
    - metadata


- models:
    - metadata