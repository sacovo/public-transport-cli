# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.4] - 2022-01-25
### Added
- limit parameter in request
- unit tests
- a bit of help for the cli args
### Changed
- Moved the access of the api from main into the request module

## [0.1.3] - 2022-01-25
### Removed
- automatic paging, it caused build issues and people can pipe it on their own.
### Added
- options to activate and deactivate colored output

## [0.1.2] - 2022-01-22
### Added
- Display time until the connection leaves
- Use colors and a bit of asci to make output nicer

## [0.1.1] - 2022-01-22
### Added
- A changelog file
- Paged output of connections with `pager`

## [0.1.0] - 2022-01-21
### Added
- Structs that represent the api
- CLI to query the api and print results
