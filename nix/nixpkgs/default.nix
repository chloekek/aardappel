let
    pinned   = fromTOML (builtins.readFile ./pinned.toml);
    tarball  = fetchTarball pinned;
    config   = { };
    overlays = [ ];
in
    import tarball { inherit config overlays; }
