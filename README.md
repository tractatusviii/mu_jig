https://mmcif.wwpdb.org/

Things i'd like to improve upon (on mmcif).
1. chemical completeness  (See https://github.com/openmm/pdbfixer)
2. biological modularity (classification and access)
3. 


TODO:

### Read the mmcif format specification
https://mmcif.wwpdb.org/docs/user-guide/guide.html

### In parallel Articulate what you'd like to see that's missing in the current infrastructrue



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



## Random cif shit:

https://github.com/scipion-em/scipion-em-atomstructutils
https://github.com/PDB-REDO/cif-tools