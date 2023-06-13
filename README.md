# Third-Party Application Container

A Docker container for a Sifis-Home third-party application automatically
generated through Continuous Integration starting from code.

# Extract label from the docker image

To extract the labels from the Docker image, the script called `name` is
provided. This script gets the percentage that evaluates the quality of a
software and the manifest without pulling the image from `ghcr` hub.

Another solution, but not advised, might be run the `docker inspect` command on
the pulled image. Each application image should be start with the prefix `3pa`,
followed by the name of the docker image. Below and example which shows how to
extract **only** the labels from a docker image.

```
docker inspect --format='{{json .Config.Labels}}' ghcr.io/sifis-home/3pa_image_name
```

# First label

The first label, called `software.quality`, is a percentage which evaluates the
quality of a software.
To show this value in the marketplace, a traffic lights system needs to be
implemented.

Below an example using the 100% value.

```
Software quality

[Red icon]
[Orange icon]
[Green icon turned on] 100%
```

The title `Software quality` must be bold.

The term `turned on` means that the color associated to the percentage
must be more saturated compared to the others.

Below the list of rules to associate the percentage to the right color:

- Red --> when percentage < 60
- Orange --> when 60% <= percentage < 80%
- Green --> when percentage >= 80%

# Second label

The second label, called `manifest`, contains a `json` with the
hazards associated to each Sifis-Home API used in the application.
In the marketplace, it must be possible to visualize this json file and
download it.
