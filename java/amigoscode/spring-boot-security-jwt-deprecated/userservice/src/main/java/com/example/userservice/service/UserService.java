package com.example.userservice.service;

import com.example.userservice.domain.AppUser;
import com.example.userservice.domain.Role;

import java.util.List;

public interface UserService {
    AppUser saveUser(AppUser appUser);
    Role saveRole(Role role);
    void addRoleToUser(String username, String roleName);
    AppUser getUser(String username);
    // if you have a lot of users, don't load all of them(just a batch)
    List<AppUser>getUsers();
}
