![License: MIT License](https://img.shields.io/badge/License-mit-blue.svg)


# Cyrano

Rust library for the CYRANO fencing protocol (EFP1.1).

## ⚠️ Work in Progress

This library is currently under active development and is not yet ready for production use. API may change without notice.

## About

Cyrano is a parser and serializer for the CYRANO protocol, enabling Ethernet communication between scoring software and piste devices in fencing competitions.

The CYRANO protocol (technical name: EFP1.1) was designed in 2008 by Jean-François Nicaud and the Favero Company, in collaboration with the FIE and various fencing software/hardware manufacturers. It defines a standard way for competition management software and piste apparatuses to exchange match data, scores, timing, and fencer information over UDP.

## Features

- Parse incoming CYRANO messages from piste apparatuses
- Serialize outgoing messages to send to devices
- Support for all message types: HELLO, DISP, INFO, ACK, NAK, NEXT, PREV
- Strongly typed representation of all protocol fields

## Protocol overview

The protocol enables bidirectional communication:

- **Software → Apparatus**: Send match information (fencers, competition phase, etc.)
- **Apparatus → Software**: Send real-time updates (scores, stopwatch, lights, cards, match state, etc.)

Messages are transmitted over UDP on port 50100 using a CSV-like format with three sections: general fields, right fencer data, and left fencer data.

## References

- [CYRANO Protocol Specification v1.1](https://superfencingsystem.com/CyranoProtocol-1-1.pdf) *(October 2019)*

## License

MIT
