# Project Title

Delivery Proof – A Soroban Smart Contract for Package Delivery Confirmation on Stellar

## Project Vision

This project demonstrates a **package delivery confirmation system built on Soroban and Stellar**. It provides:
- A decentralized way to track package deliveries on-chain
- Role-based actions for senders, couriers, and recipients
- Transparent delivery status verifiable by anyone

The goal is to provide a reliable, transparent delivery confirmation system that can be extended and integrated with real-world logistics.

---

## Description

A Soroban smart contract dApp that enables package delivery confirmation on Stellar Testnet. The system tracks deliveries from creation through acceptance to final confirmation, providing all parties with verifiable proof of delivery status on the blockchain.

---

## Features

### 1. Create Delivery
- Sender creates a delivery with a unique ID
- Stores sender, recipient, and package description
- Status starts as "created"

### 2. Accept Delivery
- Courier accepts the delivery
- Status changes to "accepted"
- Courier address is recorded

### 3. Confirm Delivery
- Courier confirms the delivery is complete
- Status changes to "confirmed"
- Recipient can verify completion

### 4. Verify Delivery
- Recipient can verify delivery status
- Anyone can check delivery status on-chain
- Transparent and auditable

### 5. Get Delivery
- Retrieve full delivery details
- Returns sender, recipient, courier, package description, and status

---

## Contract

- **Network**: Stellar Testnet
- **Contract ID**: [CB4GWKXI45BFG7I5UUTZ4IWUBIFSG3X3FO7PIB4QOWUQ5OPVFBPFKGJ3](https://stellar.expert/explorer/testnet/tx/f0cdbb5e50270eac9eb8e57aeae5e7519ba362466ffdd04c6c183eaa2a82aacc)

![screenshot](https://i.ibb.co/bRRdXbW9/image.png)

---

## Future Scopes

### 1. Multi-Courier Support
- Allow multiple couriers to handle different legs of delivery

### 2. Time Stamping
- Add timestamps for each delivery stage

### 3. Proof of Delivery
- Allow recipient to provide signature confirmation on-chain

### 4. Frontend dApp
- Build a web interface for senders, couriers, and recipients to interact

### 5. Escrow Payments
- Integrate payment release upon successful delivery confirmation

### 6. Delivery Ratings
- Add a rating system for couriers after delivery completion

---

## Profile

- **Name:** tongtu641
