;; SPDX-License-Identifier: MIT
;; SPDX-FileCopyrightText: 2025 Jonathan D.A. Jewell
;; ECOSYSTEM.scm - Related projects and ecosystem context

(ecosystem
  (name "mobile-ai-orchestrator")
  (type "library")  ;; NOT an application
  (purpose "Platform-agnostic AI routing decisions for constrained devices")

  (position-in-ecosystem
    "This is the LIBRARY layer - provides routing intelligence.
     Applications like neurophone consume this library.
     Does NOT do inference - tells you WHERE to run inference.")

  (related-projects
    (project
      (name "neurophone")
      (url "https://github.com/hyperpolymath/neurophone")
      (relationship "consumer")
      (description "Android application that could use this library for routing")
      (differentiation
        "neurophone = Android app with sensors, LSM, ESN, JNI
         this = Platform-agnostic routing library (no sensors, no JNI)"))

    (project
      (name "echomesh")
      (url "https://github.com/hyperpolymath/echomesh")
      (relationship "complementary")
      (description "Conversation context preservation across sessions"))

    (project
      (name "oblibeny")
      (url "https://github.com/hyperpolymath/oblibeny")
      (relationship "inspiration")
      (description "Safety-critical programming language concepts"))

    (project
      (name "polyglot-db-mcp")
      (url "https://github.com/hyperpolymath/polyglot-db-mcp")
      (relationship "sibling")
      (description "MCP server for databases - similar adapter pattern"))

    (project
      (name "polyglot-container-mcp")
      (url "https://github.com/hyperpolymath/polyglot-container-mcp")
      (relationship "sibling")
      (description "MCP server for containers - similar adapter pattern")))

  (what-this-is
    "A Rust library that decides WHERE to run AI inference:
     - Local (on-device SLM like llama.cpp)
     - Remote (cloud API like Claude)
     - Blocked (safety rules prevent execution)

     Provides: Expert system, Router, Context manager, Neural routing (MLP/ESN/SNN)
     Does NOT provide: Actual inference, sensor processing, Android bindings")

  (what-this-is-not
    "- NOT an Android app (see neurophone for that)
     - NOT a complete AI system (just routing decisions)
     - NOT inference engine (brings your own llama.cpp/Claude)
     - NOT sensor processing (brings your own sensors)")

  (future-integration
    "neurophone could adopt this library for intelligent routing:
     - neurophone handles: sensors → LSM → ESN → LLM integration
     - this library handles: should query go local or cloud?"))
