# Third-Party Application Container

A Docker container for a SIFIS-Home third-party application automatically
generated through Continuous Integration starting from code.

The name of a SIFIS-Home third-party application image starts with the prefix `3pa_` 
and is followed by the name of the application. 
In this repository, the name of the application is `lamp` and the name of the
docker image is `3pa_lamp`. 
The repository name is decoupled from this naming convention.


# Extract labels from the docker image

To extract the labels from the Docker image, the script `get-labels.sh` is
provided. This script retrieves the Docker image labels without pulling the 
image.
gets the percentage that evaluates the quality of a
software and the manifest without pulling the image from `ghcr` hub.

A less efficient solution to retrieve the labels is to run the `docker inspect` 
command. This solution implies that the image has already been pulled image. 
To extract the labels from the pulled image, run the following command:

```
docker inspect --format='{{json .Config.Labels}}' ghcr.io/sifis-home/3pa_image_name
```

# Labels meaning

## software.quality

This label is a percentage that represents the quality of a software.
Within the marketplace, this label is visualized with a colored cicle,
following a traffic light scheme.

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

Below the list of rules to associate the percentage with the right color:

- Red --> when percentage < 60
- Orange --> when 60% <= percentage < 80%
- Green --> when percentage >= 80%

## manifest

The second label, called `manifest`, contains a `json` with the
hazards associated to each SIFIS-Home API used in the application.
In the marketplace, it must be possible to visualize this json file and
download it.
