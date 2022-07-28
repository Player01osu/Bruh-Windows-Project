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

echo "Enter material the image is from: "
read -r MATERIAL

echo "Enter link to source: "
read -r LINK

echo "Enter width of image: "
read -r WIDTH

echo "Enter height of image: "
read -r HEIGHT

FORMATED="{\"title\":\"$TITLE\",\"author\":\"$AUTHOR\",\"op\":\"$OP\",\"tags\":[$TAGS],\"time\":$TIME,\"file_name\":\"$FILENAME\",\"source\":{\"material\":\"$MATERIAL\",\"link\":\"$LINK\"},\"resolution\":{\"width\":$WIDTH,\"height\":$HEIGHT}}"

echo $TITLE $AUTHOR $OP $TIME $TAGS $FILENAME
echo $FORMATED

set -v
curl --header "Content-Type: application/json" \
  --request POST \
  --data "$FORMATED" \
  http://localhost:7878/api/post_image & echo $AUTHOR-$TIME-$FILENAME | xclip -sel clip

prompt () {
	# prompts the user with message in $1-2 ($1 in blue, $2 in magenta) and saves the input to the variables in $REPLY and $REPLY2
	printf "\n\n\033[1;34mMove image into posts directory? [Y/n]: \033[0m"
	read -r MOVEPOST
}
prompt

prompt () {
	printf "\n\n\033[1;34mInput path the image is located (ie: ~/Pictures/stuffs/test-image.jpg OR ../other/directory): \033[0m"
	read -r IMAGEPATH
}

case $MOVEPOST in
	"n")
	return
	;;
	"N")
	return
	;;
	"y")
	prompt
	;;
	"Y")
	prompt
	;;
	"")
	prompt
	;;
esac

if [ -z $IMAGEPATH || -f $IMAGEPATH]; then
	echo post-to-db.sh: ERROR: Image path did not exist
else
	cp $IMAGEPATH ./frontend/assets/posts/$AUTHOR-$TIME-$FILENAME
fi
