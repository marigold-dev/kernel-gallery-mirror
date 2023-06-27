# This image is meant to contain the compiled rollup dir, and a script to import this dir into a kubernetes volume.
FROM ubuntu:latest

WORKDIR /app

# Copy the "rollup" directory into the image, and metadata files
COPY rollup ./rollup
COPY *.metadata ./

# Create a MD5 checksum of the "rollup" directory and writes it into a file
RUN md5sum ./rollup/installer.hex > installer.md5.metadata ; cat installer.md5.metadata

# Copy the import script into the image
COPY import-kernel-data.sh ./import-kernel-data.sh

# Make the script executable
RUN chmod +x ./import-kernel-data.sh

RUN chmod -R 755 .

RUN ls -last .

# Command to run the deployment script
CMD ["./import-kernel-data.sh"]
