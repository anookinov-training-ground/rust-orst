#!/usr/bin/Rscript
t <- read.table('values.dat', header=TRUE)
library(ggplot2)
# ggplot(t, aes(n, comparisons, colour = algorithm)) + geom_point() + geom_smooth() + scale_y_log10() + scale_x_log10()
ggplot(t, aes(n, time, colour = algorithm)) + geom_point() + scale_y_log10() + scale_x_log10() + geom_line()