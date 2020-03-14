#!/usr/bin/env python3

import configparser
import os
import pathlib
import re
import subprocess
import sys


#
# Check python version
#

if not (sys.version_info.major == 3 and sys.version_info.minor >= 6):
    print("This script requires Python 3.6 or higher!")
    print("You are using Python {}.{}.".format(sys.version_info.major, sys.version_info.minor))
    sys.exit(1)


HOME = pathlib.Path.home()
XDG_CONFIG_HOME = os.getenv("XDG_CONFIG_HOME", HOME / ".config")
XDG_CONFIG_HOME = pathlib.Path(XDG_CONFIG_HOME)

#
# TMux + TTM Configuration integration
#

def _get_user_tmux_conf():
    """
    Get the absolute path to the users configuration file of TMux.
    This includes a prioritized search on different locations.
    """
    # Define the different possible locations.
    xdg_location = XDG_CONFIG_HOME / "tmux/tmux.conf"
    default_location = HOME / ".tmux.conf"

    # Search for the correct configuration file by priority.
    return xdg_location if xdg_location.is_file() else default_location


def _get_conf_content(full=True):
    """"""
    content = ""

    for conf in map(pathlib.Path, ("/etc/tmux.conf", _get_user_tmux_conf())):
        if conf.is_file():
            content += open(conf).read() + "\n"

    if full:
        for conf in _sourced_files():
            if conf.is_file():
                content += open(conf).read() + "\n"

    return content


def _sourced_files():
    """"""
    regex = re.compile(r"^\s*source(-file)?(?P<name>.+)$", re.MULTILINE)

    files = [m.group("name") for m in regex.finditer(_get_conf_content(False))]
    files = set([re.sub("\"|\'", "", f).strip() for f in files])
    return [pathlib.Path(f) for f in files]


def ttm_list_theme_options_helper():
    """"""
    # Extact options from configuration files
    rex = re.compile(
        r"^\s*set(-option)?\s+-g\s+(?P<theme>@theme.+)$", re.MULTILINE)
    opts = [m.group("theme") for m in rex.finditer(_get_conf_content())]

    # Parse options
    rex = re.compile(r"(?P<opt>^@theme[\w\-]*)\s+(?P<val>.+)", re.MULTILINE)
    opts = "\n".join(opts)
    opts = {m.group('opt'): m.group('val') for m in rex.finditer(opts)}
    opts = {o: re.sub("\"|\'", "", v).strip() for o, v in opts.items()}

    return opts


#
# TPM + TTM Configuration integration
#


ttm_options = ttm_list_theme_options_helper()
current_theme = ttm_options.get("@theme")


def resolve_tpm_theme(theme):
    """"""
    xdg_location = XDG_CONFIG_HOME / "tmux" / "plugins"
    default_location = HOME / ".tmux" / "plugins"

    plugins = xdg_location if xdg_location.is_dir() else default_location

    return plugins / (theme + ".ttm")


def load_theme(file_path):
    """Load theme file"""
    config = configparser.ConfigParser(interpolation=None)
    config.read(file_path)
    config = {s: dict(config.items(s)) for s in config.sections()}
    return config


theme = load_theme(resolve_tpm_theme(current_theme)).get('theme')


def set_window_option(option, value):
    """"""
    value = value.strip("\"").strip("\'")
    res = subprocess.run(
        ["tmux", "set-window-option", "-g", option, value],
        stdout=subprocess.PIPE, stderr=subprocess.PIPE
    )

    if res.returncode != 0:
        print(res.stdout, file=sys.stdout)
        print(res.stderr, file=sys.stderr)


for opt, value in theme.items():
    set_window_option(opt, value)
