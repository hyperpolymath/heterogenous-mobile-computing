;; heterogenous-mobile-computing - Guix Package Definition
;; Run: guix shell -D -f guix.scm

(use-modules (guix packages)
             (guix gexp)
             (guix git-download)
             (guix build-system cargo)
             ((guix licenses) #:prefix license:)
             (gnu packages base))

(define-public heterogenous_mobile_computing
  (package
    (name "heterogenous-mobile-computing")
    (version "0.1.0")
    (source (local-file "." "heterogenous-mobile-computing-checkout"
                        #:recursive? #t
                        #:select? (git-predicate ".")))
    (build-system cargo-build-system)
    (synopsis "Hybrid AI orchestration for constrained mobile platforms")
    (description "Mobile AI Orchestrator - intelligent AI routing system for
constrained mobile devices using safety-first, offline-first architecture with
zero unsafe blocks. Part of the RSR ecosystem.")
    (home-page "https://github.com/hyperpolymath/heterogenous-mobile-computing")
    ;; Dual-licensed: user may choose either MIT or AGPL-3.0-or-later
    (license (list license:expat license:agpl3+))))

;; Return package for guix shell
heterogenous_mobile_computing
