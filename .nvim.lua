require('lspconfig').rust_analyzer.setup({
  settings = {
    ["rust-analyzer"] = {
      procMacro = {
        ignored = {
          ["leptos_macro"] = { "server" }
        }
      },
      cargo = {
        features = "all"
      },
      rustfmt = {
        overrideCommand = { "leptosfmt", "--stdin", "--rustfmt" },
      },
    }
  }
})
