# Third-Party Applications Generator

It automatically generates Docker containers for some SIFIS-Home
third-party applications through Continuous Integration using code as starting
point.

# Extract labels from the docker image

## Shell script

To extract the labels from the Docker image, the `get-labels.sh` script has been
created. This script retrieves Docker image labels without pulling an
image from the `ghcr` hub.

To get labels for the `lamp` image on `amd64`:

```
./get-labels.sh gchr.io/sifis-home/lamp-amd64 latest
```

The second option, `latest`, represents the most recent version of the image.

To get labels for the `lamp` image on `arm64`:

```
./get-labels.sh gchr.io/sifis-home/lamp-arm64 latest
```

Both the two labels are printed on shell.

## Docker inspect

A less efficient solution to retrieve the labels is to run the `docker inspect`
command. This solution implies that the image has already been pulled image.
To extract the labels from the pulled image, run the following command:

```
docker inspect --format='{{json .Config.Labels}}' ghcr.io/sifis-home/lamp-amd64
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

- Red --> when percentage < 60%
- Orange --> when 60% <= percentage < 80%
- Green --> when percentage >= 80%

## manifest

The second label, called `manifest`, contains a `json` with the
hazards associated to each SIFIS-Home API used in the application.
In the marketplace, it must be possible to visualize this json file and
download it.
