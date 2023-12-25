FROM rust:1.74.1-slim-bullseye as base


FROM base as build
WORKDIR /app

COPY . .

RUN cargo build --release

RUN chmod +x ./target/release/lessannoyingcrm-salesnavigator

CMD ["./", "target", "release", "lessannoyingcrm-salesnavigator"]

# ################################################################################
# # Create a final stage for running your application.
# #
# # The following commands copy the output from the "build" stage above and tell
# # the container runtime to execute it when the image is run. Ideally this stage
# # contains the minimal runtime dependencies for the application as to produce
# # the smallest image possible. This often means using a different and smaller
# # image than the one used for building the application, but for illustrative
# # purposes the "base" image is used here.
# FROM base AS final

# # Create a non-privileged user that the app will run under.
# # See https://docs.docker.com/develop/develop-images/dockerfile_best-practices/#user
# ARG UID=10001
# RUN adduser \
#     --disabled-password \
#     --gecos "" \
#     --home "/nonexistent" \
#     --shell "/sbin/nologin" \
#     --no-create-home \
#     --uid "${UID}" \
#     appuser
# USER appuser

# # Copy the executable from the "build" stage.
# COPY --from=build /bin/hello.sh /bin/

# # What the container should run when it is started.
# ENTRYPOINT [ "/bin/hello.sh" ]
