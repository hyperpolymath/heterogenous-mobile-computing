; SPDX-License-Identifier: PMPL-1.0-or-later
;; guix.scm — GNU Guix package definition for heterogenous-mobile-computing
;; Usage: guix shell -f guix.scm

(use-modules (guix packages)
             (guix build-system gnu)
             (guix licenses))

(package
  (name "heterogenous-mobile-computing")
  (version "0.1.0")
  (source #f)
  (build-system gnu-build-system)
  (synopsis "heterogenous-mobile-computing")
  (description "heterogenous-mobile-computing — part of the hyperpolymath ecosystem.")
  (home-page "https://github.com/hyperpolymath/heterogenous-mobile-computing")
  (license ((@@ (guix licenses) license) "PMPL-1.0-or-later"
             "https://github.com/hyperpolymath/palimpsest-license")))
