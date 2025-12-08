;;; STATE.scm - Mobile AI Orchestrator Project State
;;; Format: Guile Scheme S-expressions
;;; Last Updated: 2025-12-08
;;; Repository: hyperpolymath/heterogenous-mobile-computing

;;;============================================================================
;;; METADATA
;;;============================================================================

(define-module (state mobile-ai-orchestrator)
  #:export (project-state))

(define metadata
  '((format-version . "1.0.0")
    (state-schema . "hyperpolymath/state.scm")
    (created . "2025-11-22")
    (updated . "2025-12-08")
    (session-count . 1)))

;;;============================================================================
;;; PROJECT IDENTITY
;;;============================================================================

(define project
  '((name . "Mobile AI Orchestrator")
    (repository . "hyperpolymath/heterogenous-mobile-computing")
    (description . "Hybrid AI orchestration system for constrained mobile platforms combining on-device inference with intelligent routing to remote APIs")
    (version . "0.1.0")
    (license . "MIT + Palimpsest-0.8")
    (compliance . "RSR Bronze")))

;;;============================================================================
;;; CURRENT POSITION
;;;============================================================================

(define current-position
  '((phase . "Phase 2+ Complete")
    (status . "in-progress")
    (completion . 65)
    (summary . "Phase 1 MVP fully implemented and tested. Phase 2+ advanced features (reservoir computing, MLP routing, SNN, persistence) implemented but not yet integrated into production pipeline.")

    (achievements
      ((name . "Phase 1 MVP")
       (status . "complete")
       (completion . 100)
       (items
         ("Expert system with rule-based safety (PRIVACY_001, PRIVACY_002, SAFETY_001, RESOURCE_001)")
         ("Heuristic router (keyword and length-based decisions)")
         ("In-memory context manager with project switching")
         ("Orchestrator pipeline coordination")
         ("CLI interface (interactive and single-query modes)")
         ("41 passing tests with >90% coverage")
         ("Zero unsafe blocks - fully memory-safe")
         ("RSR Bronze compliance achieved")))

      ((name . "Phase 2+ Features")
       (status . "implemented-not-integrated")
       (completion . 100)
       (items
         ("Echo State Network reservoir (1000-neuron liquid state)")
         ("MLP neural router (384-dim input, 2 hidden layers)")
         ("Spiking Neural Networks for wake detection")
         ("Training infrastructure (online + batch modes)")
         ("SQLite persistence layer")
         ("Benchmarking suite (Criterion)")
         ("3 example applications"))))

    (codebase-stats
      ((total-lines . 4198)
       (source-files . 12)
       (test-count . 41)
       (examples . 3)
       (benchmarks . 3)
       (documentation-words . 10000)))))

;;;============================================================================
;;; ROUTE TO MVP v1 (PHASE 3 INTEGRATION)
;;;============================================================================

(define mvp-v1-route
  '((target . "Phase 3: Integration & Training")
    (goal . "Replace heuristic routing with trained MLP, integrate reservoir for context compression")
    (estimated-completion . 80)

    (milestones
      ((id . "M1")
       (name . "MLP Router Integration")
       (status . "pending")
       (priority . "high")
       (tasks
         ("Wire MLP into Router::route() as alternative to heuristics"
          "Add feature flag to switch between heuristic and neural routing"
          "Create training data collection mechanism"
          "Implement online learning from user feedback")))

      ((id . "M2")
       (name . "Sentence Transformer Integration")
       (status . "pending")
       (priority . "high")
       (tasks
         ("Replace bag-of-words with sentence-transformers embeddings"
          "Evaluate rust-bert vs ONNX runtime for mobile"
          "Optimize model size for 2-4GB RAM constraint"
          "Add embedding caching for repeated queries")))

      ((id . "M3")
       (name . "Reservoir Training Pipeline")
       (status . "pending")
       (priority . "medium")
       (tasks
         ("Collect real conversation data for training"
          "Train reservoir on conversation patterns"
          "Integrate reservoir state into ContextManager"
          "Validate 10x compression claim (1000 turns -> 100-dim)")))

      ((id . "M4")
       (name . "Hardware Deployment")
       (status . "pending")
       (priority . "low")
       (tasks
         ("Deploy SNN on DSP/NPU hardware"
          "Benchmark on MediaTek Dimensity 900"
          "Test ARM NEON intrinsics optimization"
          "Profile battery impact"))))))

;;;============================================================================
;;; CURRENT BLOCKERS & ISSUES
;;;============================================================================

(define blockers
  '((critical . ())

    (high
      (((id . "ISSUE-001")
        (title . "No real inference backend")
        (description . "Currently returns placeholder responses. Need to integrate llama.cpp or similar for actual local inference.")
        (impact . "Cannot demonstrate real AI capabilities")
        (resolution . "Integrate llama-rs or llama.cpp FFI bindings"))

       ((id . "ISSUE-002")
        (title . "Sentence transformer not integrated")
        (description . "MLP router expects 384-dim embeddings but we're using mock vectors. Need real sentence-transformers.")
        (impact . "Neural routing cannot work with real queries")
        (resolution . "Add rust-bert or ONNX runtime dependency"))))

    (medium
      (((id . "ISSUE-003")
        (title . "No training data")
        (description . "MLP and reservoir need real conversation data to train on. Currently using synthetic test data.")
        (impact . "Cannot validate learned routing decisions")
        (resolution . "Implement telemetry collection, use anonymized user feedback"))

       ((id . "ISSUE-004")
        (title . "Persistence not connected to orchestrator")
        (description . "SQLite persistence layer exists but isn't wired into the main pipeline.")
        (impact . "Context lost between sessions")
        (resolution . "Add persistence calls to Orchestrator::new() and process()"))))

    (low
      (((id . "ISSUE-005")
        (title . "No Android/iOS build tested")
        (description . "Cross-compilation documented but not CI-tested")
        (impact . "Mobile deployment unverified")
        (resolution . "Add cross-compilation to CI matrix"))

       ((id . "ISSUE-006")
        (title . "Benchmarks not baselined")
        (description . "Criterion benchmarks exist but no baseline established for regression detection")
        (impact . "Cannot detect performance regressions")
        (resolution . "Run cargo bench --save-baseline in CI"))))))

;;;============================================================================
;;; QUESTIONS FOR USER
;;;============================================================================

(define questions
  '(((id . "Q1")
     (priority . "high")
     (question . "Which local LLM backend should we prioritize?")
     (context . "Options: llama.cpp (via FFI), llama-rs (pure Rust), candle (HuggingFace Rust), ONNX Runtime")
     (tradeoffs . "llama.cpp is most mature but requires unsafe FFI. Pure Rust options are safer but less optimized."))

    ((id . "Q2")
     (priority . "high")
     (question . "What embedding model for sentence-transformers?")
     (context . "Need 384-dim embeddings. Options: all-MiniLM-L6-v2 (22MB), gte-small (33MB), e5-small-v2 (33MB)")
     (tradeoffs . "Smaller models faster but less accurate. Need to balance mobile constraints vs routing quality."))

    ((id . "Q3")
     (priority . "medium")
     (question . "Should we collect anonymized telemetry for training?")
     (context . "MLP router needs real routing decisions to learn from. Could collect: query length, keywords (hashed), routing decision, latency, user feedback.")
     (tradeoffs . "Privacy vs model improvement. Could make opt-in with local-only option."))

    ((id . "Q4")
     (priority . "medium")
     (question . "Target mobile platform for first deployment?")
     (context . "Options: Android (largest market), iOS (App Store review), Linux Mobile (PinePhone - open ecosystem)")
     (tradeoffs . "Android has JNI complexity. iOS requires Swift bridging. Linux Mobile is easiest but smallest user base."))

    ((id . "Q5")
     (priority . "low")
     (question . "Should Phase 4 features (MoE, RAG, Knowledge Graph) be separate crates?")
     (context . "Current monolith works but Phase 4 adds significant complexity.")
     (tradeoffs . "Monorepo simpler for development. Workspace allows independent versioning."))))

;;;============================================================================
;;; LONG-TERM ROADMAP
;;;============================================================================

(define roadmap
  '((phases
      (((id . "phase-1")
        (name . "MVP Foundation")
        (status . "complete")
        (completion . 100)
        (timeframe . "Completed 2025-11-22")
        (deliverables
          ("Expert system with rule-based safety"
           "Heuristic router"
           "In-memory context manager"
           "Orchestrator pipeline"
           "CLI interface"
           "Test suite (>90% coverage)"
           "RSR Bronze compliance")))

       ((id . "phase-2")
        (name . "Neural Components")
        (status . "complete")
        (completion . 100)
        (timeframe . "Completed 2025-11-22")
        (deliverables
          ("Echo State Network reservoir"
           "Multi-Layer Perceptron router"
           "Spiking Neural Networks"
           "Training infrastructure"
           "SQLite persistence"
           "Benchmarking suite")))

       ((id . "phase-3")
        (name . "Integration & Training")
        (status . "in-progress")
        (completion . 0)
        (timeframe . "Current")
        (deliverables
          ("Integrate MLP with production router"
           "Replace bag-of-words with sentence-transformers"
           "Train reservoir on real conversation data"
           "Deploy SNN on DSP/NPU hardware"
           "Establish performance baselines")))

       ((id . "phase-4")
        (name . "Advanced Intelligence")
        (status . "planned")
        (completion . 0)
        (timeframe . "Future")
        (deliverables
          ("Mixture of Experts (specialized routing)"
           "Bayesian decision engine"
           "RAG system (document retrieval)"
           "Knowledge graph (project relationships)"
           "On-device fine-tuning"
           "Reinforcement learning from feedback")))

       ((id . "phase-5")
        (name . "Production Deployment")
        (status . "planned")
        (completion . 0)
        (timeframe . "Future")
        (deliverables
          ("Android app release"
           "iOS app release"
           "Linux mobile packages"
           "Edge device support"
           "RSR Silver compliance")))))

    (vision
      ((short-term . "Demonstrate intelligent local/remote routing with real inference")
       (medium-term . "Cross-session context preservation solving context-switching hell for 60+ projects")
       (long-term . "Fully autonomous mobile AI assistant that learns from user patterns while preserving privacy")))))

;;;============================================================================
;;; CRITICAL NEXT ACTIONS
;;;============================================================================

(define next-actions
  '(((priority . 1)
     (action . "Integrate llama.cpp or llama-rs for real local inference")
     (rationale . "Cannot validate routing decisions without actual inference")
     (depends-on . "Q1 decision"))

    ((priority . 2)
     (action . "Add sentence-transformer embeddings (rust-bert or ONNX)")
     (rationale . "MLP router requires real 384-dim embeddings")
     (depends-on . "Q2 decision"))

    ((priority . 3)
     (action . "Wire persistence layer into Orchestrator")
     (rationale . "Enable cross-session context preservation")
     (depends-on . ()))

    ((priority . 4)
     (action . "Add feature flag for MLP vs heuristic routing")
     (rationale . "Allow A/B testing of routing strategies")
     (depends-on . ()))

    ((priority . 5)
     (action . "Establish benchmark baselines in CI")
     (rationale . "Detect performance regressions automatically")
     (depends-on . ()))))

;;;============================================================================
;;; DEPENDENCIES & RELATED PROJECTS
;;;============================================================================

(define dependencies
  '((upstream
      (("llama.cpp" . "Local LLM inference")
       ("sentence-transformers" . "Query embeddings")
       ("rusqlite" . "SQLite persistence")
       ("serde" . "Serialization")))

    (related-projects
      (("echomesh" . "Conversation context preservation - feeds into reservoir design")
       ("oblibeny" . "Safety-critical language - informs expert system rules")
       ("upm" . "Universal Project Manager - source of 60+ project problem")
       ("cadre" . "CRDT state management - potential persistence backend")))))

;;;============================================================================
;;; SESSION NOTES
;;;============================================================================

(define session-notes
  '((current-session
      ((date . "2025-12-08")
       (focus . "STATE.scm creation for project continuity")
       (accomplishments . ("Created comprehensive STATE.scm checkpoint"))
       (discoveries . ())
       (blockers-encountered . ())))))

;;;============================================================================
;;; EXPORT
;;;============================================================================

(define project-state
  `((metadata . ,metadata)
    (project . ,project)
    (current-position . ,current-position)
    (mvp-v1-route . ,mvp-v1-route)
    (blockers . ,blockers)
    (questions . ,questions)
    (roadmap . ,roadmap)
    (next-actions . ,next-actions)
    (dependencies . ,dependencies)
    (session-notes . ,session-notes)))

;;; End of STATE.scm
