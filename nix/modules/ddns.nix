args@{ lib, pkgs, config, ... }:

{
  options.k.services.k-ddns = {
    enable = lib.mkEnableOption "enable k-ddns service / timer";
    pkg = lib.mkOption {
      type = lib.package;
      description = "k-ddns package";
      default = args.inputs.k-ddns.packages.${pkgs.system}.default;
    };
    tokenFile = lib.mkOption {
      type = lib.types.path;
      description = "The file to load the token from";
    };
    domains = lib.mkOption {
      type = lib.types.listOf lib.types.str;
      description = "The domains to update";
      default = [];
    };
    ipv4 = lib.mkEnable "enable IPv4";
    ipv6 = lib.mkEnable "enable IPv6";
  };

  config = lib.mkIf config.k.services.ddns.enable {
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
        cfg = config.k.services.k-ddns;
        domains =
          builtins.concatStringsSep "," cfg.domains;
          tokenPath = cfg.tokenFile;
      in
      {
        script = ''
            set -e
            TOKEN=$(cat ${tokenPath}) \
            ${cfg.pkg}/bin/k-ddns \
              -d ${domains} \
          ''
          ++ (if cfg.ipv4 then " -4 auto" else " -4 nope")
          ++ (if cfg.ipv6 then " -6 auto" else " -6 nope")
          ;
      };
  };
}
