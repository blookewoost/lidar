## Intent
This project is a foray into the world of LiDAR and data processing with Rust.

First things first, we need some data! Thankfully, NOAA has us covered with their awesome *Data Access Viewer*  
This tool allows us to get a dataset of public LiDAR data for any coastal region we are interested in exploring.

Let's check out this cool looking island off the coast of Georgia. ![Screenshot](images/tybee_snip.png)

The standard filetype for storing and distributing LiDAR data are **.laz** files.  
These are losslessly compressed files containing point cloud data. The uncompressed equivalent is a **.las** file.  

Plenty of open source tools exist for decompressing **.laz** files. I am using *pdal* for this project.  

The standard format for a .las file is defined here: https://www.asprs.org/a/society/committees/standards/asprs_las_format_v12.pdf  
Most importantly, the files contain (x,y,z) point data and a terrain classification.

