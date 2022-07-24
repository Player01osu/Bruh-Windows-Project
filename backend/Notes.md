
# Generate Posts

- Get request going to accept two fields
	- Page number
		- going to start off at 1
		- each page generates 10 new posts
		- each page builds off last request
	- Sort method
		- default is sort new
- Frontend going to send out request
	- /api/view-posts/{page number}/{sort method}
	- so by default
		- /api/view-posts/1/new
		- this will generate 10 posts
	- scrolling down to bottom will result in new request
		- /api/view-posts/2/new
		- this will generate 20 posts
