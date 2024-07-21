import React from 'react';
import { View, Text, StyleSheet, ImageBackground, TextInput, TouchableOpacity } from 'react-native';
import { MaterialIcons } from '@expo/vector-icons';

export default function ChatScreen() {
  return (
    <ImageBackground source={require('../assets/pexels-fotios-photos-2304253.jpg')} style={styles.background}>
      <View style={styles.overlay}>
        <View style={styles.container}>
          <Text style={styles.title}>Chat with EcoBot</Text>
          <Text style={styles.description}>
            "Hey there! Ready to explore the limitless possibilities with me, your AI buddy? 🚀 Let's unlock the power of Internet Computer, where decentralized apps, smart contracts, DeFi, and more await! Together, we'll dive into a world of innovation and endless opportunities. Let's get started on our journey to the future!"
          </Text>
        </View>
        <View style={styles.inputContainer}>
          <TextInput style={styles.input} placeholder="Message EcoBot" placeholderTextColor="#aaa" />
          <TouchableOpacity style={styles.sendButton}>
            <MaterialIcons name="send" size={24} color="white" />
          </TouchableOpacity>
        </View>
      </View>
    </ImageBackground>
  );
}

const styles = StyleSheet.create({
  background: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  overlay: {
    flex: 1,
    width: '100%',
    backgroundColor: 'rgba(0,0,0,0.5)',
  },
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    paddingHorizontal: 20,
  },
  title: {
    fontSize: 24,
    color: 'white',
    fontWeight: 'bold',
    textAlign: 'center',
    marginBottom: 20,
  },
  description: {
    fontSize: 18,
    color: 'white',
    textAlign: 'center',
    marginVertical: 20,
  },
  inputContainer: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
    padding: 10,
    backgroundColor: 'rgba(0,0,0,0.7)',
    borderTopWidth: 1,
    borderColor: '#555',
  },
  input: {
    flex: 1,
    color: 'white',
    paddingHorizontal: 10,
    paddingVertical: 5,
  },
  sendButton: {
    marginLeft: 10,
    padding: 10,
    backgroundColor: '#003f5c',
    borderRadius: 25,
  },
});
