# Sensors Tool

A comprehensive WebAssembly tool for accessing and visualizing device sensors via standard Web APIs.

## Overview
Demonstrates how to access and visualize:
- **Accelerometer**: 3-axis acceleration data with real-time graph visualization
- **Gyroscope**: 3-axis rotation rate data with real-time graph visualization
- **Magnetometer**: 3-axis magnetic field data with compass heading calculation
- **Device Orientation**: Alpha, Beta, Gamma orientation angles
- **Ambient Light Sensor**: Light level in lux (if available)
- **Proximity Sensor**: Proximity detection (if available)
- **Camera**: Live camera stream

## Features
- **Real-time Graphing**: Visualizes accelerometer and gyroscope data (X, Y, Z) on HTML Canvas
- **Multiple Sensor Support**: Handles accelerometer, gyroscope, magnetometer, orientation, light, and proximity sensors
- **Permission Handling**: Helper logic for iOS 13+ permission requests
- **Camera Stream**: Displays live camera feed
- **Visual Indicators**: Light level bar, proximity indicator, compass heading
- **Responsive Design**: Works on both desktop and mobile devices

## Sensor Data Visualization
- **Accelerometer Graph**: Real-time line graph showing X (red), Y (teal), Z (yellow) acceleration values
- **Gyroscope Graph**: Real-time line graph showing X, Y, Z rotation rates
- **Magnetometer**: Displays X, Y, Z magnetic field values and calculated compass heading
- **Device Orientation**: Shows alpha (rotation around Z), beta (front-back tilt), gamma (left-right tilt)
- **Ambient Light**: Visual bar indicator showing light level
- **Proximity**: Visual indicator that changes color and size based on proximity

## Browser Compatibility
- **DeviceMotionEvent/DeviceOrientationEvent**: Widely supported on mobile devices
- **Generic Sensor API** (AmbientLightSensor, ProximitySensor): Requires HTTPS and may not be available in all browsers
- **Camera**: Requires user permission and HTTPS in most browsers

## Dependencies
- `web-sys`: For `DeviceMotionEvent`, `DeviceOrientationEvent`, `MediaStream`, etc.
- `wasm-bindgen`: For JavaScript interop
- `wasm-bindgen-futures`: For async operations
- `js-sys`: For JavaScript object manipulation
