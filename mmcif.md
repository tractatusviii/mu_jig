# What's the problem that needs solving?

I don't want to capture the next revolution: i want it to land in my
pocket.

Let us assume the next revolution will happen at the confluence of CryoET,
protein and rna folding and, maybe, LLMs' ability to reason.

### What that world might look like:

- gargantuan-scale images at angstrom-resolution

- digital twins
- molecular force fields
- near-perfect polymer folding
- ligand/binding affinity prediction



### What our part in it might be:

Ok, it seems like there is there a shared nexus at the boundary of:
- cryoem
- MD
- atomic/crystallographic data encoding
- sequence

The implicit proposition here is that by having a [common substrate](what is it?
a format? a framework? a type system? a library? an application?) the friction
is reduced...

**Answer this**: Who benefits from this acceleration?


Poetry:

- "modularity at biological hierarchy boundaries"


# Who is going to use it?

# Who is going to pay you for it?

# What is the job here that won't need doing in 5-10 years?

# What is the job that will need doing doing in 5 years but doesn't exist now?










#-------------------------------------------------------------------------
# Notes
https://github.com/chanzuckerberg/cryoet-data-portal-backend/blob/main/schema/v1.1.0/metadata-docs/TiltSeries.md
https://github.com/chanzuckerberg/cryoet-data-portal-backend/tree/main/schema/v1.1.0
https://ngff.openmicroscopy.org/about/index.html




https://datascience.codata.org/articles/10.5334/dsj-2016-003

#---------------------
### Complaining
format spec is ugly, says biopython developer: https://stackoverflow.com/a/11686524/10697358
People routinely write their own parsers to reconcile these and curse the design choices and lack of documentation.
Ex. here are guys from model-angelo ( GNN for model building that chenwei looked at) fixing this on their onw : https://github.com/3dem/model-angelo/issues/51
Here is Deep Mind implementing their own parser https://huggingface.co/spaces/simonduerr/ProteinMPNN/blob/f969e9cfb6f11ba299c7108aadff124e9cf38b1f/alphafold/alphafold/data/mmcif_parsing.py
Here the  gemmi guy trying to connect his work to chimerax and stumbling over this : https://mail.cgl.ucsf.edu/mailman/archives/list/chimerax-users@cgl.ucsf.edu/thread/XOO3G5MUOHLQBHSVF2NENWYNG3SOUF2O/
https://bioinformatics.stackexchange.com/questions/14210/pdb-residue-numbering


#---------------------
# Format Specs

https://www.iucr.org/resources/cif
https://datascience.codata.org/articles/10.5334/dsj-2016-003


## Formal specs
https://www.iucr.org/resources/cif/cif2
https://www.iucr.org/resources/cif/spec/version1.1

https://legacy.ccp4.ac.uk/newsletters/newsletter37/13_harvest.html



## Implementations and random docs
https://www.ccp4.ac.uk/html/cciflib.html
http://comcifs.github.io/cif_api/index.html


## History of
https://www.ccp4.ac.uk/html/mmcifformat.html

- Implementation of Data Harvesting in the CCP4 Suite : https://legacy.ccp4.ac.uk/newsletters/newsletter37/13_harvest.html

"HARVESTING" :https://www.ccp4.ac.uk/html/harvesting.html
