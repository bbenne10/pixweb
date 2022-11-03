{ system, nixpkgs, pre-commit, rustPkg }:
let
  pkgs = nixpkgs.legacyPackages.${system};
  pch = pkgs.python39Packages.pre-commit-hooks;
in pre-commit.lib.${system}.run {
  src = ./.;
  hooks = {

    # commit message formatting
    check_commit_message = {
      enable = true;
      name = "check basic commit message style";
      stages = [ "commit-msg" ];
      entry = "${pkgs.perl}/bin/perl checks/commit-check";
    };

    # generic on-disk file checking
    check_added_large_files = {
      enable = true;
      name = "check for added large files";
      entry = "${pch}/bin/check-added-large-files";
    };

    fix_byte_order_marker = {
      enable = true;
      name = "fix byte order marker";
      entry = "${pch}/bin/fix-byte-order-marker";
    };

    check_case_conflict = {
      enable = true;
      name = "check case conflict";
      entry = "${pch}/bin/check-case-conflict";
    };

    check_merge_conflict = {
      enable = true;
      name = "check merge conflict";
      entry = "${pch}/bin/check-merge-conflict";
    };

    end_of_file_fixer = {
      enable = true;
      name = "fix end of files";
      entry = "${pch}/bin/end-of-file-fixer";
    };

    forbid_new_submodules = {
      enable = true;
      name = "forbid new submodules";
      entry = "${pch}/bin/forbid-new-submodules";
    };

    trailing_whitespace = {
      enable = true;
      name = "trailing whitespace";
      entry = "${pch}/bin/trailing-whitespace-fixer";
    };

    # nix
    nixfmt.enable = true;
    nix-linter.enable = true;

    # rust
    our_rustfmt = {
      name = "rustfmt";
      description = "Format Rust code.";
      enable = true;
      entry = "${rustPkg}/bin/cargo-fmt fmt -- --check --color always";
      files = "\\.rs$";
      pass_filenames = false;
    };

    our_clippy = {
      name = "clippy";
      description = "Lint Rust code.";
      enable = true;
      entry = "${rustPkg}/bin/cargo-clippy clippy";
      files = "\\.rs$";
      pass_filenames = false;
    };
  };
}
