#/usr/bin/bash
#
# From rust code
# cell side length is 1
#
#splacesurf recommends particle radius be 1.4 to 1.6 times the "SPH particles radius"
#
splashsurf reconstruct --particle-radius 1 --smoothing-length 1.2 --cube-size 0.2 pc.json -o out.obj

