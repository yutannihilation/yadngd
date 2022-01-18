
<!-- README.md is generated from README.Rmd. Please edit that file -->

# yadngd: Yet Another /dev/null Graphic Device using extendr

This repository is for testing
[extendr/extendr#360](https://github.com/extendr/extendr/pull/360)

<!-- badges: start -->
<!-- badges: end -->

## Installation

``` r
remotes::install_github("yutannihilation/yadngd")
```

## Usage

``` r
library(yadngd)
yadngd()
#> 🎉🍕🍰📺🍓✨🍣🐈🎿🎉🍕🍰📺🍓✨🍣🐈🎿
#> 
#>    ◆祝◆ device activated!!! ◆祝◆   
#> 
#> 🎉🍕🍰📺🍓✨🍣🐈🎿🎉🍕🍰📺🍓✨🍣🐈🎿
#> [1] 2

dev.list()
#>     png yadndgd 
#>       2       3

# do nothing
plot(1)

dev.off()
#> png 
#>   2
```
