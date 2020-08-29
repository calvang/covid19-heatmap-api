# Deploying a Rocket/Rust Application to Heroku

While Rust does not have an official Heroku buildpack, the emk/rust buildpack is quite popular and stable. To create a Heroku repository with the buildpack, run:

`heroku create <appname> --buildpack em/rust`

Since this particular application uses Python scripts as well, it also needs the Python buildpack. To add this buildpack, run:

`heroku buildpacks:add --index 1 heroku/python`

You can view the list of buildpacks for the app by running:

`heroku buildpacks`
