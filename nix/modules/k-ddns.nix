args@{ lib, pkgs, config, ... }:

let
  cfg = config.k.services.k-ddns;
in
{
  options.k.services.k-ddns = {
    enable = lib.mkEnableOption "enable k-ddns service / timer";
    pkg = lib.mkOption {
      type = lib.types.package;
      description = "k-ddns package";
      default = args.inputs.k-ddns.packages.${pkgs.system}.default;
    };
    tokenFile = lib.mkOption {
      type = lib.types.path;
      description = "The file to load the token from";
    };
    domain = lib.mkOption {
      type = lib.types.str;
      description = "The domain / hostname to update";
      default = [ ];
    };
    ipv4 = lib.mkEnableOption "enable IPv4";
    ipv6 = lib.mkEnableOption "enable IPv6";
  };

  config = lib.mkIf config.k.services.k-ddns.enable {
    systemd.timers."k-ddns" = {
      wantedBy = [ "timers.target" ];
      description = "k-ddns service timer";
      timerConfig = {
        OnBootSec = "1min";
        OnActiveSec = "5min";
      };
    };

    systemd.services."k-ddns" =
      let
        # domains = builtins.concatStringsSep "," cfg.domains;
        domain = cfg.domain;
        tokenPath = cfg.tokenFile;
      in
      {
        script =
          ''
            set -e
            DYNV6_TOKEN=$(cat ${tokenPath}) \
            DOMAIN=${domain} \
            ${cfg.pkg}/bin/k-ddns \
          ''
          + (if cfg.ipv4 then " -4 auto" else " -4 nope")
          + (if cfg.ipv6 then " -6 auto" else " -6 nope")
        ;
      };
  };
}
