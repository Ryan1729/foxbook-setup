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

echo 'adding the user with following command seems to leave the user with an unusable password. If that happens run `passwd {0}` to set your password, but you should be able to just run it interactively to have it work.'
echo 'adduser {0}'

echo 'after that next step is:'
echo 'foxbook-setup 1 | ash'

echo 'note that the next step needs you to type the exact same username.'
"#, name);
                }
                None => {
                    println!("Step 0 requires a username");
                    std::process::exit(2);
                }
            }
        }
        Some("1") => {
            match args.next() {
                Some(name) => {
                    print!("{}", r#"
apk add sudo nano
awk '/root ALL=\(ALL\) ALL/ {{ print; print "{0} ALL=(ALL) ALL"; next }}1' /etc/sudoers > temp && cat temp > /etc/sudoers && rm temp
su {0}
cd ~

"#, name);
                }
                None => {
                    println!("Step 1 requires the username from step 0");
                    std::process::exit(2);
                }
            }
        }
        Some("2") => {
            print!("{}", r###"
echo '# Prepend the prompt with the return code of the last run command 
# and show username instead of computer name
export PS1="\$? \u:\w$ "
' > ~/.profile

source ~/.profile

#
#    set up graphical environment for next boot
#

echo 'for some reason this built in script seemed to fail if run inside another script. You'll have to run it yourself.'

echo 'sudo setup-xorg-base'

echo 'after that next step is:'
echo 'foxbook-setup 3 | ash'

"###r);
        }
        Some("3") => {
           print!("{}", r#"
#
#    allow all offical non-edge repositories
#

echo 'We need to allow downloading from the community repositories, and the config for this is stored in
the /etc/apk/repositories file. You just need to uncomment the single line contaiing the non-edge
community repo. Scripting this turned out to be suprisingly compilicated to do reliably, so you will again 
have to do it yourself.'

echo 'nano /etc/apk/repositories'

echo 'next step is check the README with:'
echo 'foxbook-setup'

echo 'and once you know what to do run:'
echo 'foxbook-setup 4 | ash'
"#r);
        }
        Some("4") => {
           print!("{}", r#"
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
        Some("5") => {
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
