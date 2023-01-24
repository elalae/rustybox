# Rustybox

The code takes arguments from the CLI in order to carry out the tasks.

The first argument is registered as the command.

I implemented the following commands:

pwd - read the content using env::current_dir().unwrap() then printed it

echo [-n] - used args().skip(2/3).collect::<Vec<_>>().join(" ")) to collect the arguments and print them 

cat - made a function that includes fs::read_to_string(path) and stores the content into a variable "contents" then matches it to print the content or exit with an error

mkdir- used fs::create_dir_all(path) to create a dir

rmdir - used md.is_dir() and match fs::remove_dir(path) to remove a dir

rm [-r] [-d] - similar the rmdir but when is without options it verifies if given  path is file or dir

touch [-c] - creates a file or not if has the option  -c, if there is option -a it uses copy(path, path) to finish its task

ln [-s] - used std::os::unix::fs::symlink for -s option and fs::hard_link(from, to) for hard link

mv - used rename(path, path2) to impement the command  
chmod - fs::set_permissions to set permissions with numbers 

cp- mostly used fs::rename(from, to), fs::copy(from, to) plus some path formating

ls [-a] - variable b is used to find out if file name is hidden or not, mostly used  fs::read_dir(dir) and printed the files

