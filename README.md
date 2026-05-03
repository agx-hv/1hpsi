# 🌫️ 1h PSI

A mobile webapp for viewing real-time air quality in Singapore using 1-hour PM2.5 readings from NEA data.

It converts raw PM2.5 values into a **PSI-equivalent scale** using NEA’s official calculation method so the readings are easier to understand at a glance.

---

## 💡 What this app does

- Shows real-time **1-hour PM2.5 readings**
- Converts them into a PSI-equivalent value using NEA’s formula
- Breaks data down by region:
  - North, South, East, West, Central
- Shows an **overall average across all regions**
- Provides three views:
  - 📊 Summary (latest readings)
  - 📈 Chart (trend over time)
  - 📋 Table (full data)

---

## ⚠️ Important note

This differs from the official PSI value from National Environment Agency.

- Official PSI uses multiple pollutants (PM2.5, PM10, SO2, CO, O3, NO2)
- This webapp uses **hourly PM2.5 readings only**
- Values are calculated using NEA’s official linear interpolation method
- This shows **how PM2.5 changes hour by hour**, while official PSI includes more pollutants and smoothing over time

You can also verify that if you take a **24-hour average of the readings for each region**, the values closely match NEA’s reported PSI trends for those regions.

For official updates, refer to:
https://www.haze.gov.sg

---

## 🧮 How it works

- Gets 1-hour PM2.5 readings from NEA
- Converts them into PSI-equivalent values using NEA’s formula
- Groups results by region
- Calculates an overall average

---

## 🌐 Platform

- Mobile-first webapp
- Designed mainly for mobile WebView use
- iOS support may be added in the future

---

## 📦 Releases

Releases are provided as **APK files only**.

Download the latest APK from the Releases section.

---

## 🔧 Building from source

### Requirements

- Rust (via rustup): https://rustup.rs/
- Android SDK
- Android NDK
- Java
- Tauri Android setup configured

---

## 🚀 Run in development

cargo tauri dev

---

## 📱 Build for Android (development)

cargo tauri android dev

---

## 📦 Build for Android (release APK)

cargo tauri android build

---

## 🌐 Frontend

The frontend is fully static (HTML / CSS / JS) and is bundled directly into the Tauri build. No npm or frontend build tools are required.

---

## 📦 Output

The build process generates an **APK file** in the Android output directory. This APK is used for releases.

---

## 📚 Data source

- NEA 1-hour PM2.5 readings  
  https://www.haze.gov.sg/resources/1-hr-pm2.5-readings

- PSI reference formula  
  https://www.haze.gov.sg/docs/default-source/faq/computation-of-the-pollutant-standards-index-(psi).pdf

---

## 🧠 Purpose

To provide a simple, fast view of air quality in Singapore by focusing on PM2.5, which is often the main pollutant during haze periods.
