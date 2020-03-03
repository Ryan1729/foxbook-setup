fn main() {
    let mut args = std::env::args();

    // exe name
    args.next();

    match args.next().as_ref().map(|s| s.as_str()) {
        Some("0") => {
            match args.next() {
                Some(name) => {
                    print!(r#"
echo '# Prepend the prompt with the return code of the last run command 
# and show username instead of computer name
export PS1="\$? \u:\w$ "
' > ~/.profile

source ~/.profile

#
#    getting wifi working on next boot
#

rc-update add wpa_supplicant default
# awk example came from here: https://stackoverflow.com/a/18276534
awk '/need localmount/ {{ print; print "\tneed wpa_supplicant"; next }}1' /etc/init.d/networking > temp
cat temp > /etc/init.d/networking

rc-update -u

rm temp

#
#    add new user
#

adduser {0}
apk add sudo nano
awk '/root ALL=\(ALL\) ALL/ {{ print; print "{0} ALL=(ALL) ALL"; next }}1' /etc/sudoers > temp && cat temp > /etc/sudoers && rm temp
su {0}
cd ~
"#, name);
                }
                None => {
                    println!("Step 0 requires a username");
                    std::process::exit(2);
                }
            }
        }
        Some("1") => {
            print!("{}", r###"
echo '# Prepend the prompt with the return code of the last run command 
# and show username instead of computer name
export PS1="\$? \u:\w$ "
' > ~/.profile

source ~/.profile

#
#    set up graphical environment for next boot
#

setup-xorg-base

#
#    allow all offical non-edge repositories
#

# We need to allow downloading from the community repositories, and the config for this is stored in
# the /etc/apk/repositories file. You just need to uncomment the single line contaiing the non-edge
# community repo. This script will uncomment all lines starting with "#h" and which have a v elsewhere
# which in this case, does the same job. This is admittedly a little ugly, and potentially fragile,
# but it gets the job done, at least right now.
awk '/^#h.*v.*/ {{ print substr($1, 2, length($1)); next }}1' /etc/apk/repositories > temp
cat temp > /etc/apk/repositories

# tell apk we updated the repositories file
apk update

#
#    install window manager and firefox
#

sudo apk add firefox-esr ttf-dejavu i3wm dmenu i3status

# start i3 when x11 starts
echo "
exec i3" > ~/.xinitrc

# start up graphical env to get the config written
startx

"###);
        }
        Some("2") => {
           print!("{}", r#"
die () {
    echo $1; exit 1
}

test ! -e temp || die 'Temp file already exists. Move it so it does not get overwritten.'

sudo touch temp
sudo chmod 666 temp

# make firefox startup on boot
sudo awk '/bindsym XF86AudioMicMute/ { print; print "\n# This is a fox book\nexec firefox"; next }1' ~/.config/i3/config > temp
sudo cat temp > ~/.config/i3/config

rm temp

echo '# Prepend the prompt with the return code of the last run command 
# and show username instead of computer name
export PS1="\$? \u:\w$ "

# start x11 when appropriate
if [[ -z $DISPLAY ]] && [[ $(tty) = /dev/tty1 ]]; then
         startx
fi' > ~/.profile
"#);
        }
        Some(step) => {
            println!("Unknown step number \"{}\"", step);
            std::process::exit(1);
        }
        None => {
            print!("version {}\n\n{}", env!("CARGO_PKG_VERSION"), std::include_str!("../README.md"));        
        }
    }
}
