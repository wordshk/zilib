# Yes. Weird import. Importing the rust stuff
from .zilib import *

def _package_path():
    # Whoever designed this API...

    # Note: importlib.resources was introduced in python3.7, resources.files in
    # python3.9, and resources.path is deprecated since 3.11. Our code in
    # words.hk is running on 3.8. FML.
    from importlib import resources
    if hasattr(resources, 'path'):
        with resources.path(__package__, '__init__.py') as p:
            return p.parent
    else:
        return resources.files(__package__)

def _initialize_resources():
    package_path = _package_path()

    # Initialize the rust library
    zilib.initialize_data("CantoneseWordListWithJyutping", str(package_path.joinpath('lists', 'wordslist.csv')))
    zilib.initialize_data("CantoneseCharListWithJyutping", str(package_path.joinpath('lists', 'charlist.json')))
    zilib.initialize_data("RadicalLabelToChars", str(package_path.joinpath('lists', 'CJKRadicals.txt')))
    zilib.initialize_data("EnglishVariants", str(package_path.joinpath('lists', 'english_variants.json')))
    # Unihan is not included in the package. So we don't initialize it here.

_initialize_resources()

def wordshk_charset():
    """Returns the words.hk character set. Used by words.hk as a reference to
    what words.hk considers as "canonical" character forms."""

    package_path = _package_path()
    import json
    with package_path.joinpath('lists', 'wordshk_charset.json').open('r') as f:
        return json.load(f)

def wordshk_variantmap():
    """Returns a dictionary that maps variants to their canonical forms as
    given by wordshk_charset()."""

    package_path = _package_path()
    import json
    with package_path.joinpath('lists', 'wordshk_charset.json').open('r') as f:
        return json.load(f)

def wordshk_autoconvertmap():
    """'Safe' (not-so-controversial) map of variants to words.hk canonical
    characters. In words.hk, we automatically convert these values to the
    canonical form. We also automatically display the variants when users view
    any entry with these characters."""

    package_path = _package_path()
    import json
    with package_path.joinpath('lists', 'wordshk_autoconvert.json').open('r') as f:
        return json.load(f)
