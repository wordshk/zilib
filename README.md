# zilib

A library written in rust with functions for Han character (and some English
word) processing.

(Not to be confused with zlib, the compression library)

## Scope

This library contains character-level or word-level language functions used by
words.hk. We may expand the scope later. Documentation and examples will come
soon.

Originally these functions were implemented in Python in the words.hk codebase,
but we have ported them to Rust, with optional python bindings - (see
https://pypi.org/project/zilib/ )

As of writing 2024-02, this repository is under active development. Consider
the state "unstable" until this notice is removed.

Pull requests are welcome as long as the changes are released under the same
terms as in LICENSE.md. However, from experience (with our response times to
email enquiries), we may sometimes take a while to respond. (Please do not
interpret it as disinterest, we may simply have forgotten to check our
emails...)

If you plan to open pull requests to make changes to data files in `list/`,
please read the section Principles and Philosophies below. Also note that those
files are under different copyrights/licenses.

## License

See [LICENSE.md](LICENSE.md)

## Principles and Philosophies

### Decision Problem Avoidance

In our experience, we have found that trying to classify things into clear-cut
categories often leads to more problems than it solves. This is explained in
detail by Lau (2019):

> There are several classes of problems that have been classified as 'decision
> problems' by our team. A decision problem is something that requires a
> judgement, a clear-cut A or B answer. These problems are often not helpful
> when compiling a dictionary. For example, drawing a line between vegetables
> and fruits is a decision problem. Some are fairly easy, e.g. orange is
> definitely a fruit, lettuce is definitely a vegetable; some are less trivial,
> e.g. tomato. These decision problems do not really give much information to
> dictionary users. In the latter case, it is preferable for a dictionary to
> explain the cultural and botanical classifications of tomato, instead of
> forcing them into inflexible categories.
>
> *Lau, C.-M. (2019). Building Cantonese dictionaries using crowdsourcing
> strategies: The words.hk project. In A. W.-B. Tso (Ed.), Digital humanities and
> new ways of teaching (pp. 89-107). Singapore: Springer.*

Note that in zilib, we work on lists where characters/words are either *in* the
list or *not*. These are "decision problems" that are unavoidable due to the
nature of the library. We do not have the luxury of further explaining the
*apparently* clear-cut decisions (at least not within the API), therefore such
decisions may look somewhat arbitrary at the edges. This is an unfortunate fact
when we are building a library that is used by many people who may be each
operating under slightly different circumstances, contexts and requirements.

Nuances and edge cases can often by handled by further complicating the
"interface" (i.e. the library API), but this makes the library harder to use.
It is a delicate exercise to balance between a simple and easy-to-use library
and a library that can handle all edge cases, since the latter will force the
users of the API to battle with the inherent complexities of language
processing, in particular the politics and "fractal-esque" aspects of languages
and dialects.

As such, while we try to maintain quality of the reference lists, we decline to
claim to be authoritative on the decisions made, i.e. we are not here to set
standards. As a practical note, any changes to our version (i.e. "fork") of the
library needs to be usable by our own applications (namely words.hk), and we
may not be able to accommodate all requests. For the principles/philosophies of
words.hk, see https://words.hk/base/about/ . Given the generally open nature of
this project (zilib), we encourage users to fork the library and make their own
changes if they have requirements that are incompatible with ours.
