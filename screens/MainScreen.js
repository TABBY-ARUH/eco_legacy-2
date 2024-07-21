import React from 'react';
import { View, Text, StyleSheet, ImageBackground, ScrollView, TouchableOpacity, Image } from 'react-native';

export default function ProjectScreen({ navigation }) {
  return (
    <ImageBackground source={require('../assets/pexels-fotios-photos-2304253.jpg')} style={styles.background}>
      <View style={styles.overlay}>
        <View style={styles.container}>
          <Text style={styles.title}>Explore Sustainability Projects</Text>
          <ScrollView style={styles.scrollContainer}>
            <Text style={styles.yearTitle}>Year 2000</Text>
            <View style={styles.projectRow}>
              <View style={styles.projectCard}>
                <Image source={require('../assets/pexels-fotios-photos-2304253.jpg')} style={styles.projectImage} />
                <Text style={styles.projectTitle}>Project 1.0</Text>
                <Text style={styles.projectDescription}>Renewable energy initiative</Text>
              </View>
              <View style={styles.projectCard}>
                <Image source={require('../assets/pexels-fotios-photos-2304253.jpg')} style={styles.projectImage} />
                <Text style={styles.projectTitle}>Project 2.0</Text>
                <Text style={styles.projectDescription}>Green building practices</Text>
              </View>
              <View style={styles.projectCard}>
                <Image source={require('../assets/pexels-casnafu-23628331.jpg')} style={styles.projectImage} />
                <Text style={styles.projectTitle}>Project 3.0</Text>
                <Text style={styles.projectDescription}>Urban Greening</Text>
              </View>
            </View>
            <Text style={styles.yearTitle}>Year 2010</Text>
            <View style={styles.projectRow}>
              <View style={styles.projectCard}>
                <Image source={require('../assets/pexels-thgusstavo-4749478.jpg')} style={styles.projectImage} />
                <Text style={styles.projectTitle}>Project 4.0</Text>
                <Text style={styles.projectDescription}>Public Transportation and Cycling Infrastructure</Text>
              </View>
              <View style={styles.projectCard}>
                <Image source={require('../assets/pexels-paggiarofrancesco-938044.jpg')} style={styles.projectImage} />
                <Text style={styles.projectTitle}>Project 5.0</Text>
                <Text style={styles.projectDescription}>Waste management and Recycling programs</Text>
              </View>
              <View style={styles.projectCard}>
                <Image source={require('../assets/pexels-thisisengineering-3912372.jpg')} style={styles.projectImage} />
                <Text style={styles.projectTitle}>Project 6.0</Text>
                <Text style={styles.projectDescription}>Sustainable Agriculture</Text>
              </View>
            </View>
          </ScrollView>
          <TouchableOpacity style={styles.button} onPress={() => {}}>
            <Text style={styles.buttonText}>Explore Other Projects ➔</Text>
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
    paddingHorizontal: 20,
    paddingTop: 60,
  },
  title: {
    fontSize: 24,
    color: 'white',
    fontWeight: 'bold',
    textAlign: 'center',
    marginBottom: 20,
  },
  scrollContainer: {
    flex: 1,
  },
  yearTitle: {
    fontSize: 20,
    color: 'white',
    marginBottom: 10,
    marginLeft: 10,
  },
  projectRow: {
    flexDirection: 'row',
    justifyContent: 'space-around',
    marginBottom: 20,
  },
  projectCard: {
    width: '30%',
    backgroundColor: 'rgba(255, 255, 255, 0.2)',
    borderRadius: 10,
    padding: 10,
    alignItems: 'center',
  },
  projectImage: {
    width: '100%',
    height: 100,
    borderRadius: 10,
    marginBottom: 10,
  },
  projectTitle: {
    fontSize: 16,
    color: 'white',
    fontWeight: 'bold',
    textAlign: 'center',
  },
  projectDescription: {
    fontSize: 14,
    color: 'white',
    textAlign: 'center',
  },
  button: {
    backgroundColor: 'rgba(255, 255, 255, 0.8)',
    paddingVertical: 10,
    paddingHorizontal: 20,
    borderRadius: 25,
    alignItems: 'center',
    justifyContent: 'center',
    marginVertical: 20,
  },
  buttonText: {
    fontSize: 18,
    color: '#000',
  },
});
