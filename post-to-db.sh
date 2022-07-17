#!/bin/sh


prompt () {
# prompts the user with message in $1-2 ($1 in blue, $2 in magenta) and saves the input to the variables in $REPLY and $REPLY2
printf "\033[1;34mAuthor: \nOP: \nTags Wrapped in quotes (ex: \"nuts\", \"fart\", ...): \nFile name: \n%s\033[1;34m: \033[0m" "$1"
read -r TITLE
}
prompt "Title"

prompt () {
# prompts the user with message in $1-2 ($1 in blue, $2 in magenta) and saves the input to the variables in $REPLY and $REPLY2
printf "\n\n\033[1;34mTitle: \033[0m%s\033[1;34m
OP:
Tags Wrapped in quotes (ex: \"nuts\", \"fart\", ...):
File name:
%s\033[1;34m: \033[0m" "$TITLE" "$1"
read -r AUTHOR
}
prompt "Author"

prompt () {
# prompts the user with message in $1-2 ($1 in blue, $2 in magenta) and saves the input to the variables in $REPLY and $REPLY2
printf "\n\n\033[1;34mTitle: \033[0m%s\033[1;34m
Author: \033[0m%s\033[1;34m
Tags Wrapped in quotes (ex: \"nuts\", \"fart\", ...):
File name:
%s\033[1;34m: \033[0m" "$TITLE" "$AUTHOR" "$1"
read -r OP
}
prompt "OP"

TIME=$(date +%s)

prompt () {
# prompts the user with message in $1-2 ($1 in blue, $2 in magenta) and saves the input to the variables in $REPLY and $REPLY2
printf "\n\n\033[1;34mTitle: \033[0m%s\033[1;34m
Author: \033[0m%s\033[1;34m
OP: \033[0m%s\033[1;34m
File name:
%s\033[1;34m: \033[0m" "$TITLE" "$AUTHOR" "$OP" "$1"
read -r TAGS
}
prompt "Tags Wrapped in quotes (ex: \"nuts\", \"fart\", ...)"

prompt () {
# prompts the user with message in $1-2 ($1 in blue, $2 in magenta) and saves the input to the variables in $REPLY and $REPLY2
printf "\n\n\033[1;34mTitle: \033[0m%s\033[1;34m
Author: \033[0m%s\033[1;34m
OP: \033[0m%s\033[1;34m
Tags Wrapped in quotes (ex: \"nuts\", \"fart\", ...): \033[0m%s\033[1;34m
%s\033[1;34m: \033[0m" "$TITLE" "$AUTHOR" "$OP" "$TAGS" "$1"
read -r FILENAME
}
prompt "File name"

FORMATED="{\"title\":\"$TITLE\",\"author\":\"$AUTHOR\",\"op\":\"$OP\",\"tags\":[$TAGS],\"time\":$TIME,\"file_name\":\"$FILENAME\"}"

echo $TITLE $AUTHOR $OP $TIME $TAGS $FILENAME
echo $FORMATED

set -v
curl --header "Content-Type: application/json" \
  --request POST \
  --data "$FORMATED" \
  http://localhost:7878/api/post_image & echo $AUTHOR-$TIME-$FILENAME | xclip -sel clip
