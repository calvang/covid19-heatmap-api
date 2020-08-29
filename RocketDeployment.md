# Deploying a Rocket/Rust Application to Heroku

While Rust does not have an official Heroku buildpack, the emk/rust buildpack is quite popular and stable. To create a Heroku repository with the buildpack, run:

`heroku create <appname> --buildpack em/rust`

Since this particular application uses Python scripts as well, it also needs the Python buildpack. To add this buildpack, run:

`heroku buildpacks:add --index 1 heroku/python`

You can view the list of buildpacks for the app by running:

`heroku buildpacks`

You will likely need to set the Rocket port in the `Procfile`, which you need to create. See this project's procfile for a working example. The executable will be located in `target/release/<appname>`, but it will be run from the `/app` directory on the Heroku servers.
