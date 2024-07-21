import * as React from 'react';
import { NavigationContainer } from '@react-navigation/native';
import { createStackNavigator } from '@react-navigation/stack';
import WelcomeScreen from '../screens/WelcomeScreen';
import MainScreen from '../screens/MainScreen';
import ProjectScreen from '../screens/ProjectScreen';
import ChatScreen from '../screens/ChatScreen';
import TranslationScreen from '../screens/TranslationScreen';

const Stack = createStackNavigator();

export default function AppNavigator() {
  return (
    <NavigationContainer>
      <Stack.Navigator initialRouteName="Welcome">
        <Stack.Screen name="Welcome" component={WelcomeScreen} />
        <Stack.Screen name="Main" component={MainScreen} />
        <Stack.Screen name="Project" component={ProjectScreen} />
        <Stack.Screen name="Chat" component={ChatScreen} />
        <Stack.Screen name="Translation" component={TranslationScreen} />
      </Stack.Navigator>
    </NavigationContainer>
  );
}
