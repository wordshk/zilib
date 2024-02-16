# Yes. Weird import. Importing the rust stuff
from zilib.zilib import *
from importlib import resources

# Initialize the rust library
package_path = resources.files(__package__)
zilib.initialize_data("CantoneseWordListWithJyutping", str(package_path.joinpath('lists', 'wordslist.csv')))
zilib.initialize_data("CantoneseCharListWithJyutping", str(package_path.joinpath('lists', 'charlist.json')))
zilib.initialize_data("RadicalLabelToChars", str(package_path.joinpath('lists', 'CJKRadicals.txt')))
zilib.initialize_data("EnglishVariants", str(package_path.joinpath('lists', 'english_variants.json')))
# Unihan is not included in the package. So we don't initialize it here.
